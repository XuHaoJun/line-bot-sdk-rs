const fs = require("fs");
const path = require("path");
const yaml = require("js-yaml");

/**
 * Post-process generated Rust code to create proper enum implementations
 * for schemas that had discriminators
 */

/**
 * Convert schema name to snake_case for file names
 * Handles acronyms properly (e.g., URIAction -> uri_action, not u_r_i_action)
 */
function toSnakeCase(str) {
  return str
    // Insert underscore before capital letters that follow lowercase letters
    .replace(/([a-z])([A-Z])/g, "$1_$2")
    // Insert underscore before capital letters that are followed by lowercase
    // (handles acronyms like "URI" in "URIAction")
    .replace(/([A-Z])([A-Z][a-z])/g, "$1_$2")
    .toLowerCase();
}

/**
 * Parse a Rust struct file to extract field information and detect inline enums
 */
function parseStructFields(filePath) {
  if (!fs.existsSync(filePath)) {
    return null;
  }

  const content = fs.readFileSync(filePath, "utf8");
  const fields = [];
  const inlineEnums = new Set();
  let actualStructName = null;

  // Extract the actual struct name from the file
  const structMatch = content.match(/pub struct (\w+)/);
  if (structMatch) {
    actualStructName = structMatch[1];
  }

  // Detect inline enum definitions
  const enumRegex = /pub enum (\w+) \{/g;
  let enumMatch;
  while ((enumMatch = enumRegex.exec(content)) !== null) {
    inlineEnums.add(enumMatch[1]);
  }

  // Match field definitions in the struct
  // Find all serde rename attributes and their positions
  const attrPattern = /#\[serde\(rename\s*=\s*"([^"]+)"[^\]]*\)\]\s+pub\s+(\w+):\s*/gs;
  let attrMatch;
  const attrPositions = [];
  
  while ((attrMatch = attrPattern.exec(content)) !== null) {
    attrPositions.push({
      serdeName: attrMatch[1],
      fieldName: attrMatch[2],
      start: attrMatch.index,
      typeStart: attrMatch.index + attrMatch[0].length,
    });
  }
  
  // For each field, parse the type by finding the next comma at the same nesting level
  for (let i = 0; i < attrPositions.length; i++) {
    const attr = attrPositions[i];
    const nextAttrPos = i + 1 < attrPositions.length ? attrPositions[i + 1].start : content.length;
    
    // Find the comma that ends this field (not inside <>)
    let depth = 0;
    let typeEnd = attr.typeStart;
    for (let j = attr.typeStart; j < nextAttrPos; j++) {
      const char = content[j];
      if (char === '<') depth++;
      else if (char === '>') depth--;
      else if (char === ',' && depth === 0) {
        typeEnd = j;
        break;
      }
    }
    
    const fieldType = content.substring(attr.typeStart, typeEnd)
      .replace(/\s+/g, ' ')
      .trim();
    
    const isOptional = fieldType.startsWith("Option<");
    
    fields.push({
      serdeName: attr.serdeName,
      fieldName: attr.fieldName,
      fieldType,
      attributes: content.substring(attr.start, typeEnd + 1),
      isOptional,
    });
  }

  return { fields, inlineEnums, actualStructName };
}

/**
 * Extract type names from a field type (handles generics)
 */
function extractTypeNames(fieldType) {
  const types = [];
  // Extract all type names, including those in generics
  const matches = fieldType.match(/[A-Z]\w+/g);
  if (matches) {
    types.push(...matches.filter(t => !['Option', 'Vec', 'Box', 'HashMap'].includes(t)));
  }
  return types;
}

/**
 * Check if a field type references any inline enums
 */
function hasInlineEnumReference(fieldType, allInlineEnums) {
  const typeNames = extractTypeNames(fieldType);
  return typeNames.some(typeName => allInlineEnums.has(typeName));
}

/**
 * Add models:: prefix to custom types
 */
function addModelsPrefix(fieldType) {
  // Don't add prefix for:
  // - Primitive types (i32, i64, f32, f64, bool, String, etc.)
  // - std types (Option, Vec, Box, HashMap, etc.)
  // - Already prefixed types (models::, std::, crate::)
  
  if (
    /^(i8|i16|i32|i64|i128|u8|u16|u32|u64|u128|f32|f64|bool|String|str)$/.test(fieldType) ||
    /^(Option|Vec|Box|std::|models::|crate::|HashMap)/.test(fieldType)
  ) {
    return fieldType;
  }

  // For generic types, recursively process the inner type
  if (fieldType.includes("<")) {
    return fieldType.replace(/([A-Z]\w+)(?=<|,|\)|>)/g, (match) => {
      if (/^(Option|Vec|Box|HashMap|std)$/.test(match)) {
        return match;
      }
      return `models::${match}`;
    });
  }

  // Add models:: prefix to custom types
  return `models::${fieldType}`;
}

/**
 * Generate enum variant from struct fields
 */
function generateEnumVariant(variantName, serdeRename, fields) {
  if (!fields || fields.length === 0) {
    // Unit variant - no trailing comma, the join will add it
    return `    #[serde(rename = "${serdeRename}")]\n    ${variantName}`;
  }

  let variant = `    #[serde(rename = "${serdeRename}")]\n    ${variantName} {\n`;
  
  for (const field of fields) {
    // Add the serde attribute
    const serdeAttr = `        #[serde(rename = "${field.serdeName}"`;
    if (field.isOptional) {
      variant += `${serdeAttr}, skip_serializing_if = "Option::is_none")]\n`;
    } else {
      variant += `${serdeAttr})]\n`;
    }
    
    // Add models:: prefix to custom types
    const fieldType = addModelsPrefix(field.fieldType);
    variant += `        ${field.fieldName}: ${fieldType},\n`;
  }
  
  variant += `    }`;
  return variant;
}

/**
 * Generate From trait implementation
 */
function generateFromImpl(structName, variantName, fields) {
  let impl = `impl From<models::${structName}> for ${variantName} {\n`;
  impl += `    fn from(value: models::${structName}) -> Self {\n`;
  impl += `        ${variantName}::${structName} {\n`;
  
  for (const field of fields) {
    impl += `            ${field.fieldName}: value.${field.fieldName},\n`;
  }
  
  impl += `        }\n`;
  impl += `    }\n`;
  impl += `}\n`;
  
  return impl;
}

/**
 * Generate default implementation
 */
function generateDefaultImpl(enumName, variants) {
  if (variants.length === 0) {
    return `impl Default for ${enumName} {
    fn default() -> Self {
        Self
    }
}`;
  }

  // Extract first variant name
  const firstVariantLine = variants[0].split("\n")[1].trim();
  const variantName = firstVariantLine.split(/\s|{/)[0];
  const hasFields = firstVariantLine.includes("{");

  if (!hasFields) {
    return `impl Default for ${enumName} {
    fn default() -> Self {
        Self::${variantName}
    }
}`;
  }

  // For variants with fields, create a minimal default
  return `impl Default for ${enumName} {
    fn default() -> Self {
        // Using first variant with default field values
        Self::${variantName} {
            // All fields use their Default trait implementation
        }
    }
}`;
}

/**
 * Generate a complete enum implementation for a discriminated schema
 * Returns true if generated, false if skipped
 */
function generateEnumFile(
  enumName,
  discriminatorMapping,
  packagePath,
  baseFields
) {
  console.log(`  Generating enum for ${enumName}...`);

  const variants = [];
  const fromImpls = [];
  const modelsDir = path.join(packagePath, "src", "models");
  const allInlineEnums = new Set();
  const variantData = [];

  // First pass: collect all inline enums and variant data
  for (const [typeName, schemaRef] of Object.entries(discriminatorMapping)) {
    const schemaName = schemaRef.split("/").pop();
    const structFile = path.join(modelsDir, `${toSnakeCase(schemaName)}.rs`);
    
    // Parse the struct to get its fields and inline enums
    const parsed = parseStructFields(structFile);
    
    if (!parsed) {
      console.warn(`    Warning: Could not find struct file for ${schemaName}`);
      continue;
    }

    const { fields, inlineEnums, actualStructName } = parsed;
    
    // Use the actual struct name from the file, or fall back to schema name
    const structName = actualStructName || schemaName;
    
    // Collect all inline enums
    inlineEnums.forEach(e => allInlineEnums.add(e));
    
    variantData.push({
      typeName,
      structName,
      fields,
      inlineEnums,
    });
  }

  // Second pass: check if any field references inline enums
  for (const variant of variantData) {
    const hasInlineRef = variant.fields.some(field => 
      hasInlineEnumReference(field.fieldType, allInlineEnums)
    );
    
    if (hasInlineRef) {
      console.log(`    ⚠ Skipping ${enumName}: references inline enum types`);
      console.log(`      Inline enums: ${Array.from(allInlineEnums).join(', ')}`);
      console.log(`      → Keeping generated struct instead`);
      return false; // Skip generating this enum
    }
  }

  // Third pass: generate variants and From impls
  for (const variant of variantData) {
    // Generate variant
    const variantCode = generateEnumVariant(variant.structName, variant.typeName, variant.fields);
    variants.push(variantCode);

    // Generate From impl
    const fromImpl = generateFromImpl(variant.structName, enumName, variant.fields);
    fromImpls.push(fromImpl);
  }

  // Generate the complete file
  let fileContent = `/*
 * LINE Messaging API
 *
 * This document describes LINE Messaging API.
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 * Post-processed by: post-process-enums.js
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ${enumName} enum with all fields inline for each variant
/// This is automatically generated from discriminated schemas
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ${enumName} {
${variants.join(",\n")}
}

impl Default for ${enumName} {
    fn default() -> Self {
        // Using the first variant as default
        ${(() => {
          if (variantData.length === 0) return "unimplemented!()";
          const first = variantData[0];
          const hasFields = first.fields.length > 0;
          if (!hasFields) {
            return `Self::${first.structName}`;
          }
          // For variants with fields, create default values
          const fieldDefaults = first.fields.map(f => {
            if (f.isOptional) return `${f.fieldName}: None`;
            if (f.fieldType.includes("String")) return `${f.fieldName}: String::new()`;
            if (f.fieldType.includes("Box")) return `${f.fieldName}: Box::new(Default::default())`;
            if (f.fieldType.includes("Vec")) return `${f.fieldName}: Vec::new()`;
            return `${f.fieldName}: Default::default()`;
          }).join(",\n            ");
          return `Self::${first.structName} {\n            ${fieldDefaults}\n        }`;
        })()}
    }
}

// Conversion methods from struct types to enum variants
${fromImpls.join("\n")}
`;

  // Write the enum file
  const enumFile = path.join(modelsDir, `${toSnakeCase(enumName)}.rs`);
  fs.writeFileSync(enumFile, fileContent, "utf8");
  console.log(`    ✓ Generated ${enumFile}`);
  return true;
}

/**
 * Main post-processing function
 */
function postProcessEnums(specPath, packagePath) {
  console.log(`\nPost-processing enums for ${specPath}...`);

  // Read the original spec to find discriminators
  const yamlContent = fs.readFileSync(specPath, "utf8");
  const spec = yaml.load(yamlContent);

  if (!spec.components || !spec.components.schemas) {
    console.log("  No schemas found, skipping");
    return;
  }

  const schemas = spec.components.schemas;
  let generatedCount = 0;
  let skippedCount = 0;

  // Find all schemas with discriminators
  for (const [schemaName, schema] of Object.entries(schemas)) {
    if (schema.discriminator && schema.discriminator.mapping) {
      const parsed = parseStructFields(
        path.join(packagePath, "src", "models", `${toSnakeCase(schemaName)}.rs`)
      );

      const wasGenerated = generateEnumFile(
        schemaName,
        schema.discriminator.mapping,
        packagePath,
        parsed ? parsed.fields : null
      );
      
      if (wasGenerated) {
        generatedCount++;
      } else {
        skippedCount++;
      }
    }
  }

  console.log(`  ✓ Post-processed ${generatedCount + skippedCount} discriminated schema(s): ${generatedCount} generated, ${skippedCount} skipped`);
}

module.exports = { postProcessEnums };

// CLI usage
if (require.main === module) {
  const args = process.argv.slice(2);
  if (args.length < 2) {
    console.error("Usage: node post-process-enums.js <spec-path> <package-path>");
    process.exit(1);
  }

  postProcessEnums(args[0], args[1]);
}

