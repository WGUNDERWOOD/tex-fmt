#!/usr/bin/env bash
echo "Checking latex PDFs agree"
DIR="$(mktemp -d)"
cp -r ../tests/* "$DIR"
echo "$DIR"
cd "$DIR" || exit

# empty file cannot be compiled
rm -r ./empty/

echo

for TESTDIR in "$DIR"/*; do
    for file in "$TESTDIR/source"/*.tex; do
        f=$(basename "$file" .tex)
        echo "Running latex for $f.tex"
        (cd "$TESTDIR/source" && latexmk -pdflua "$f.tex")
        (cd "$TESTDIR/target" && latexmk -pdflua "$f.tex")
        (cd "$TESTDIR/source" && pdftotext -q "$f.pdf")
        (cd "$TESTDIR/target" && pdftotext -q "$f.pdf")
    done
done

echo

for TESTDIR in "$DIR"/*; do
    for file in "$TESTDIR/source"/*.tex; do
        f=$(basename "$file" .tex)
        echo "Checking PDF for $f.tex"
        diff -u "$TESTDIR/source/$f.txt" "$TESTDIR/target/$f.txt" | diff-so-fancy
    done
done

echo "$DIR"
