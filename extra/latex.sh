#!/usr/bin/env bash
echo "Checking latex PDFs agree"
DIR="$(mktemp -d)"
cp -r ../tests/* "$DIR"
echo "$DIR"
cd "$DIR" || exit

echo

for file in ./source/*.tex; do
    f=$(basename "$file" .tex)
    echo "Running latex for $f.tex"
    (cd ./source && latexmk -pdflua "$f.tex" >/dev/null 2>&1)
    (cd ./target && latexmk -pdflua "$f.tex" >/dev/null 2>&1)
    (cd ./source && pdftotext -q "$f.pdf" >/dev/null 2>&1)
    (cd ./target && pdftotext -q "$f.pdf" >/dev/null 2>&1)
done

echo

for file in ./source/*.tex; do
    f=$(basename "$file" .tex)
    echo "Checking PDF for $f.tex"
    diff -u "source/$f.txt" "target/$f.txt" | diff-so-fancy
done

echo "$DIR"
