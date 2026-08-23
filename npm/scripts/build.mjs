import fs from "node:fs";
import path from "node:path";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const npmDir = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const repoDir = path.resolve(npmDir, "..");
const wasmFile = path.join(
  repoDir,
  "target",
  "wasm32-unknown-unknown",
  "release",
  "tex_fmt.wasm",
);
const packageOutput = path.join(npmDir, "pkg");
const webOutput = path.join(repoDir, "web", "pkg");

function run(command, args, options = {}) {
  const result = spawnSync(command, args, {
    cwd: repoDir,
    encoding: "utf8",
    stdio: "inherit",
    ...options,
  });

  if (result.error) {
    throw result.error;
  }
  if (result.status !== 0) {
    throw new Error(`${command} exited with status ${result.status}`);
  }
}

run(process.execPath, [path.join(npmDir, "scripts", "check-version.mjs")]);

fs.rmSync(packageOutput, { force: true, recursive: true });
fs.rmSync(webOutput, { force: true, recursive: true });

run("cargo", [
  "build",
  "--release",
  "--locked",
  "--features",
  "wasm",
  "--lib",
  "--target",
  "wasm32-unknown-unknown",
]);

for (const [target, directory] of [
  ["web", "web"],
  ["nodejs", "node"],
]) {
  const output = path.join(packageOutput, directory);
  run("wasm-bindgen", ["--target", target, "--out-dir", output, wasmFile]);
}

const nodeBinding = path.join(packageOutput, "node", "tex_fmt.js");
const originalNodeSource = fs.readFileSync(nodeBinding, "utf8");
const nodeSource = originalNodeSource.replace(
  "`${__dirname}/tex_fmt_bg.wasm`",
  "`${__dirname}/../web/tex_fmt_bg.wasm`",
);
if (nodeSource === originalNodeSource) {
  throw new Error("Could not make the Node.js binding use the shared wasm file");
}
fs.writeFileSync(nodeBinding, nodeSource);
fs.rmSync(path.join(packageOutput, "node", "tex_fmt_bg.wasm"));
fs.rmSync(path.join(packageOutput, "node", "tex_fmt_bg.wasm.d.ts"));

fs.writeFileSync(
  path.join(packageOutput, "node", "package.json"),
  `${JSON.stringify({ type: "commonjs" }, null, 2)}\n`,
);

const wasmOpt = spawnSync("wasm-opt", ["--version"], { encoding: "utf8" });
if (wasmOpt.status === 0) {
  const file = path.join(packageOutput, "web", "tex_fmt_bg.wasm");
  run("wasm-opt", ["-Oz", "-o", file, file]);
} else {
  console.warn("wasm-opt was not found; generated WebAssembly was not optimized.");
}

fs.cpSync(path.join(packageOutput, "web"), webOutput, { recursive: true });
fs.copyFileSync(path.join(repoDir, "LICENSE"), path.join(npmDir, "LICENSE"));

console.log("Built npm WebAssembly package and web/pkg.");
