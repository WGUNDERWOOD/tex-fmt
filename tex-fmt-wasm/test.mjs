
import texFmt from './pkg/tex_fmt_wasm.js';

const TEX_DOCUMENT = `

\\documentclass{article}
\\usepackage{tikz}
\\usepackage{amsmath}
\\begin{document}
\\begin{center}

\\begin{tikzpicture}

% Draw the main rectangle for the plot area 
\\draw[thick] (0,0) rectangle (6,6);

% Draw y-axis with tick marks up to 3700 only

\\foreach \\y/\\label in {0/2900, 1.5/3100, 3/3300, 4.5/3500, 6/3700} {
\\draw (-0.1,\\y) -- (0,\\y);
% tick marks
\\node[left] at (-0.3,\\y) {\\label}; % y-axis labels moved further to the left
}

\\end{tikzpicture}

\\end{center}

\\end{document}

`;

async function main() {
  let result = texFmt.run_tex_fmt(TEX_DOCUMENT);
  console.log(result.output);
}

main();

