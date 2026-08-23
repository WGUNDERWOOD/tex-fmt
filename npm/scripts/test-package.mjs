import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const npmDir = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const temporaryDir = fs.mkdtempSync(path.join(os.tmpdir(), "tex-fmt-npm-"));

function run(command, args, cwd) {
  const result = spawnSync(command, args, {
    cwd,
    encoding: "utf8",
    env: {
      ...process.env,
      NPM_CONFIG_CACHE: path.join(temporaryDir, "npm-cache"),
    },
    stdio: "inherit",
  });
  if (result.error) {
    throw result.error;
  }
  if (result.status !== 0) {
    throw new Error(`${command} exited with status ${result.status}`);
  }
}

try {
  run("npm", ["pack", "--pack-destination", temporaryDir], npmDir);
  const archive = fs
    .readdirSync(temporaryDir)
    .find((file) => file.endsWith(".tgz"));
  if (!archive) {
    throw new Error("npm pack did not create an archive");
  }

  const consumerDir = path.join(temporaryDir, "consumer");
  fs.mkdirSync(consumerDir);
  fs.writeFileSync(
    path.join(consumerDir, "package.json"),
    `${JSON.stringify({ private: true, type: "module" }, null, 2)}\n`,
  );
  run(
    "npm",
    ["install", "--ignore-scripts", path.join(temporaryDir, archive)],
    consumerDir,
  );
  fs.writeFileSync(
    path.join(consumerDir, "test.mjs"),
    `import { format, version } from "tex-fmt";\n` +
      `const result = format("a\\n", "");\n` +
      `if (result.output !== "a\\n" || !version().startsWith("v")) process.exit(1);\n`,
  );
  run(process.execPath, ["test.mjs"], consumerDir);
  console.log("Packed tarball installed and ran in a clean project.");
} finally {
  fs.rmSync(temporaryDir, { force: true, recursive: true });
}
