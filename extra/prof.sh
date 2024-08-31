#!/usr/bin/env bash
echo "Making flamegraph profile"
DIR="$(mktemp -d)"
cp -r ../tests/* "$DIR"
cargo build --release

echo
echo -n "Test files: $(find "$DIR"/*/* | wc -l) files, "
echo -n "$(wc -l --total=only "$DIR"/source/* "$DIR"/target/*) lines, "
du -hs "$DIR" | cut -f 1
echo

flamegraph -F 10000 -- ../target/release/tex-fmt "$DIR/source/"* "$DIR/target/"*
