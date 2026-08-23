import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";

import init, {
  format,
  main,
  version,
} from "../node.mjs";

const npmDir = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const input = String.raw`\begin{document}
\begin{itemize}
\item hello
\end{itemize}
\end{document}
`;
const expected = String.raw`\begin{document}
\begin{itemize}
  \item hello
\end{itemize}
\end{document}
`;

await init();
assert.equal(format(input, "").output, expected);
assert.equal(main(input, "").output, expected);
assert.match(version(), /^v\d+\.\d+\.\d+/);

const require = createRequire(import.meta.url);
const commonJs = require("../node.cjs");
await commonJs.init();
assert.equal(commonJs.format(input, "").output, expected);

const web = await import("../pkg/web/tex_fmt.js");
const bytes = fs.readFileSync(path.join(npmDir, "pkg/web/tex_fmt_bg.wasm"));
await web.default({ module_or_path: bytes });
assert.equal(web.main(input, "").output, expected);

console.log("ESM, CommonJS, and web entry points passed.");
