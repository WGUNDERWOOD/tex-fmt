import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const npmDir = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const repoDir = path.resolve(npmDir, "..");
const packageJson = JSON.parse(
  fs.readFileSync(path.join(npmDir, "package.json"), "utf8"),
);
const cargoToml = fs.readFileSync(path.join(repoDir, "Cargo.toml"), "utf8");
const cargoVersion = cargoToml.match(
  /^version\s*=\s*"([^"]+)"/m,
)?.[1];

if (!cargoVersion) {
  throw new Error("Could not read the package version from Cargo.toml");
}

if (packageJson.version !== cargoVersion) {
  throw new Error(
    `Version mismatch: npm=${packageJson.version}, Cargo=${cargoVersion}`,
  );
}

const releaseTag = process.env.RELEASE_TAG;
if (releaseTag && releaseTag !== `v${packageJson.version}`) {
  throw new Error(
    `Release tag ${releaseTag} does not match npm version v${packageJson.version}`,
  );
}

console.log(`Version ${packageJson.version} is synchronized.`);
