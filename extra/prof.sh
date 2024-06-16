#!/usr/bin/env bash
echo "Making flamegraph profile"
DIR="$(mktemp -d)"
cp "../tests/source/phd_dissertation.tex" "$DIR"
cargo build --release

echo "Writing large test file"
for _ in {1..5}; do
    cat "$DIR/phd_dissertation.tex" >> "$DIR/large.tex"
    printf "\n\n\n" >> "$DIR/large.tex"
done

rm "$DIR/phd_dissertation.tex"

echo "Test file:"
for f in "$DIR"/*.tex; do
    echo -n "  $(basename "$f"), "
    echo -n "$(wc -l "$f" | cut --delimiter=" " --fields 1) lines, "
    du -h "$f" | cut --fields 1
done

flamegraph -F 10000 -- ../target/release/tex-fmt "$DIR"/*.tex
hyperfine --warmup 2 -n "tex-fmt" "../target/release/tex-fmt $DIR/*.tex"
