# tex-fmt WebAssembly package

This package exposes [tex-fmt](https://github.com/WGUNDERWOOD/tex-fmt), an
extremely fast LaTeX formatter written in Rust, to JavaScript through
WebAssembly. It has no runtime dependencies and includes TypeScript declarations.

## Install

```sh
npm install tex-fmt
```

## Node.js

Both ESM and CommonJS are supported. Node.js initializes the WebAssembly module
synchronously while loading the package, so `format` can be called immediately.

```js
import { format, version } from "tex-fmt";

const result = format("\\begin{document}\nHello!\n\\end{document}\n", "");
console.log(result.output);
console.log(result.logs);
console.log(version());
```

The second argument is a TOML configuration string. Pass an empty string to use
the defaults.

## Browser and bundlers

Initialize the module once before formatting:

```js
import { init, format } from "tex-fmt";

await init();
const result = format("\\section{Hello}\nText\n", "wrap = true");
```

The raw browser entry point is also exported as `tex-fmt/web`.

## API

- `format(text, config)` (also exported as `main`) returns
  `{ output: string, logs: string }`.
- `version()` returns the tex-fmt version prefixed with `v`.
- `init()` initializes WebAssembly in browsers and is a harmless async no-op in
  Node.js.
