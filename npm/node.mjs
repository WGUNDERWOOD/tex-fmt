import wasm from "./pkg/node/tex_fmt.js";

export async function init() {
  return wasm;
}

export const main = wasm.main;
export const format = wasm.main;
export const version = wasm.version;

export default init;
