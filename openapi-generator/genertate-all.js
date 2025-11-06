const fs = require("fs");
const path = require("path");
const { exec } = require("child_process");
const yaml = require("js-yaml");
const { postProcessEnums } = require("./post-process-enums");

// Create tmp directory if it doesn't exist
const TMP_DIR = "./tmp";
if (!fs.existsSync(TMP_DIR)) {
  fs.mkdirSync(TMP_DIR, { recursive: true });
}

/**
 * Flatten allOf in OpenAPI schemas to work around Rust generator limitation
 * The Rust generator doesn't support allOf, so we need to flatten it manually
 */
function flattenAllOf(schema, schemas) {
  if (!schema || typeof schema !== "object") {
    return schema;
  }

  // If this schema has allOf, flatten it
  if (schema.allOf && Array.isArray(schema.allOf)) {
    const flattened = {
      type: "object",
      properties: {},
      required: [],
    };

    // Merge all schemas in allOf
    schema.allOf.forEach((item) => {
      // Resolve $ref if present
      if (item.$ref) {
        const refName = item.$ref.split("/").pop();
        const refSchema = schemas[refName];
        if (refSchema) {
          // Merge properties (but skip 'type' if it comes from discriminator base)
          if (refSchema.properties) {
            Object.assign(flattened.properties, refSchema.properties);
          }
          // Merge required fields
          if (refSchema.required) {
            flattened.required.push(...refSchema.required);
          }
          // DO NOT copy discriminator - let child schemas be structs, not enums
        }
      } else {
        // Inline schema
        if (item.properties) {
          Object.assign(flattened.properties, item.properties);
        }
        if (item.required) {
          flattened.required.push(...item.required);
        }
      }
    });

    // Keep other properties from original schema (but exclude discriminator and allOf)
    Object.keys(schema).forEach((key) => {
      if (key !== "allOf" && key !== "discriminator") {
        flattened[key] = schema[key];
      }
    });

    // Deduplicate required fields
    flattened.required = [...new Set(flattened.required)];
    if (flattened.required.length === 0) {
      delete flattened.required;
    }

    return flattened;
  }

  return schema;
}

/**
 * Remove discriminators from schemas
 * The Rust generator's discriminator handling doesn't properly include child-specific fields
 * in enum variants. We remove discriminators so it generates structs instead.
 */
function removeDiscriminators(schemas) {
  Object.keys(schemas).forEach((schemaName) => {
    if (schemas[schemaName] && schemas[schemaName].discriminator) {
      console.log(`  Removing discriminator from ${schemaName}`);
      delete schemas[schemaName].discriminator;
    }
  });
}

/**
 * Process OpenAPI spec to flatten allOf
 */
function processOpenAPISpec(inputPath, outputPath) {
  console.log(`Processing ${inputPath}...`);

  // Read and parse YAML
  const yamlContent = fs.readFileSync(inputPath, "utf8");
  const spec = yaml.load(yamlContent);

  // Process schemas to flatten allOf
  if (spec.components && spec.components.schemas) {
    const schemas = spec.components.schemas;
    
    // Step 1: Flatten all allOf compositions
    Object.keys(schemas).forEach((schemaName) => {
      schemas[schemaName] = flattenAllOf(schemas[schemaName], schemas);
    });
    
    // Step 2: Remove discriminators to prevent broken enum generation
    removeDiscriminators(schemas);
  }

  // Write processed spec to tmp directory
  const processedYaml = yaml.dump(spec, {
    lineWidth: -1, // Don't wrap lines
    noRefs: false, // Keep $ref intact where not flattened
  });
  
  fs.writeFileSync(outputPath, processedYaml, "utf8");
  console.log(`Processed spec written to ${outputPath}`);
}

// Read the JSON file
fs.readFile("./openapi-generator/projects.json", "utf8", (err, data) => {
  if (err) {
    console.error("Error reading the JSON file:", err);
    return;
  }

  // Parse the JSON data
  const json = JSON.parse(data);
  const projects = json.projects;

  // Loop through each project and run the openapi-generator command
  projects.forEach((project) => {
    const spec = project.spec;
    const packageName = project.packageName;

    // Process the OpenAPI spec to flatten allOf
    const inputSpec = `line-openapi/${spec}`;
    const processedSpec = `${TMP_DIR}/${spec}`;
    
    try {
      processOpenAPISpec(inputSpec, processedSpec);
    } catch (error) {
      console.error(`Error processing ${spec}:`, error);
      return;
    }

    // Generate code from processed spec
    const command = `openapi-generator generate -i ${processedSpec} -g rust -o ./packages/${packageName} -c ./openapi-generator/openapi-generator-config.yaml -p "packageName=${packageName}" --git-user-id "${json["git-user-id"]}" --git-repo-id "${json["git-repo-id"]}"`;

    exec(command, (err, stdout, stderr) => {
      if (err) {
        console.error(`Error executing command for ${spec}:`, err);
        return;
      }

      console.log(`Successfully generated code for ${spec}`);
      console.log(stdout);
      if (stderr) {
        console.error(stderr);
      }

      // Post-process to generate proper enums from discriminated schemas
      try {
        postProcessEnums(inputSpec, `./packages/${packageName}`);
      } catch (error) {
        console.error(`Error post-processing enums for ${spec}:`, error);
      }
    });
  });
});