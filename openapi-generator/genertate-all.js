const fs = require("fs");
const { exec } = require("child_process");

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

    const command = `openapi-generator generate -i line-openapi/${spec} -g rust -o ./packages/${packageName} -c ./openapi-generator/openapi-generator-config.yaml -p "packageName=${packageName}" --git-user-id "${json["git-user-id"]}" --git-repo-id "${json["git-repo-id"]}"`;

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
    });
  });
});