import { readdir, readFile, rm, writeFile } from "node:fs/promises";
import { basename, extname, join } from "node:path";
import { spawnSync } from "node:child_process";
import YAML from "yaml";

const root = new URL("..", import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, "$1");
const httpContractsDir = join(root, "contracts", "http");
const serviceContractsDir = join(httpContractsDir, "services");
const mergedSpec = join(httpContractsDir, "openapi.yaml");
const apiCrateDir = join(root, "apps", "api", "crates", "api");
const serverOutputDir = join(apiCrateDir, "generated", "http");
const clientOutputDir = join(root, "apps", "web", "src", "api", "generated");

const command = process.argv[2] ?? "generate";

async function listServices() {
  const files = await readdir(serviceContractsDir);
  return files
    .filter((file) => [".yaml", ".yml", ".json"].includes(extname(file)))
    .map((file) => ({
      name: basename(file, extname(file)),
      spec: join(serviceContractsDir, file),
    }))
    .sort((a, b) => a.name.localeCompare(b.name));
}

async function readSpec(service) {
  const content = await readFile(service.spec, "utf8");

  if (extname(service.spec) === ".json") {
    return JSON.parse(content);
  }

  return YAML.parse(content);
}

function runGenerator(args) {
  const result = spawnSync("bun", ["x", "openapi-generator-cli", ...args], {
    cwd: root,
    stdio: "inherit",
    shell: process.platform === "win32",
  });

  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}

function mergeNamedMap(target, source, mapName, serviceName) {
  if (!source) {
    return;
  }

  for (const [key, value] of Object.entries(source)) {
    if (target[key] === undefined) {
      target[key] = value;
      continue;
    }

    if (JSON.stringify(target[key]) !== JSON.stringify(value)) {
      throw new Error(
        `Duplicate ${mapName} "${key}" has different definitions while merging ${serviceName}`,
      );
    }
  }
}

async function writeMergedSpec(services) {
  const merged = {
    openapi: "3.0.3",
    info: {
      title: "Poke Packer HTTP API",
      version: "0.1.0",
    },
    tags: [],
    paths: {},
    components: {},
  };

  const tagNames = new Set();
  const serverKeys = new Set();
  const servers = [];

  for (const service of services) {
    const spec = await readSpec(service);

    for (const server of spec.servers ?? []) {
      const key = JSON.stringify(server);
      if (!serverKeys.has(key)) {
        servers.push(server);
        serverKeys.add(key);
      }
    }

    for (const tag of spec.tags ?? []) {
      if (!tagNames.has(tag.name)) {
        merged.tags.push(tag);
        tagNames.add(tag.name);
      }
    }

    for (const [path, pathItem] of Object.entries(spec.paths ?? {})) {
      if (!merged.paths[path]) {
        merged.paths[path] = {};
      }

      for (const [method, operation] of Object.entries(pathItem)) {
        if (merged.paths[path][method] !== undefined) {
          throw new Error(
            `Duplicate ${method.toUpperCase()} ${path} while merging ${service.name}`,
          );
        }

        merged.paths[path][method] = operation;
      }
    }

    for (const [componentType, components] of Object.entries(spec.components ?? {})) {
      if (!merged.components[componentType]) {
        merged.components[componentType] = {};
      }

      mergeNamedMap(
        merged.components[componentType],
        components,
        `components.${componentType}`,
        service.name,
      );
    }
  }

  if (servers.length > 0) {
    merged.servers = servers;
  }

  await writeFile(mergedSpec, YAML.stringify(merged));
}

async function removeOldClientServiceOutputs(services) {
  for (const { name } of services) {
    await rm(join(clientOutputDir, name), { recursive: true, force: true });
  }
}

async function removeOldMergedServerOutput() {
  for (const path of [
    "src",
    ".openapi-generator",
    "Cargo.toml",
    ".gitignore",
    ".openapi-generator-ignore",
  ]) {
    await rm(join(serverOutputDir, path), { recursive: true, force: true });
  }
}

async function validate(services) {
  for (const service of services) {
    console.log(`Validating ${service.name}`);
    runGenerator(["validate", "-i", service.spec]);
  }

  await writeMergedSpec(services);
  console.log("Validating merged openapi");
  runGenerator(["validate", "-i", mergedSpec]);
}

async function generateServer(services) {
  await writeMergedSpec(services);
  await removeOldMergedServerOutput();

  for (const service of services) {
    const packageName = `api_http_${service.name.replaceAll("-", "_")}`;
    console.log(`Generating Rust server for ${service.name}`);
    runGenerator([
      "generate",
      "-g",
      "rust-axum",
      "-i",
      service.spec,
      "-o",
      join(serverOutputDir, service.name),
      "--additional-properties",
      `packageName=${packageName},packageVersion=0.1.0`,
    ]);
  }
}

async function generateClient(services) {
  await writeMergedSpec(services);
  await removeOldClientServiceOutputs(services);

  console.log("Generating TypeScript client from merged openapi");
  runGenerator([
    "generate",
    "-g",
    "typescript-fetch",
    "-i",
    mergedSpec,
    "-o",
    clientOutputDir,
    "--additional-properties",
    "supportsES6=true,typescriptThreePlus=true",
  ]);
}

const services = await listServices();

if (services.length === 0) {
  throw new Error(`No OpenAPI service specs found in ${serviceContractsDir}`);
}

switch (command) {
  case "validate":
    await validate(services);
    break;
  case "generate:server":
    await generateServer(services);
    break;
  case "generate:client":
    await generateClient(services);
    break;
  case "generate":
    await validate(services);
    await generateServer(services);
    await generateClient(services);
    break;
  default:
    console.error(`Unknown command: ${command}`);
    console.error("Use one of: validate, generate, generate:server, generate:client");
    process.exit(1);
}
