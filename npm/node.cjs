const wasm = require("./pkg/node/tex_fmt.js");

async function init() {
  return wasm;
}

module.exports = {
  default: init,
  init,
  main: wasm.main,
  format: wasm.main,
  version: wasm.version,
};
