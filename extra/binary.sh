#!/usr/bin/env bash
echo "Testing binary"
DIR="$(mktemp -d)"
cp -r ../tests/* "$DIR"
TRIPLET=$(gcc -dumpmachine)
BINARY="../target/$TRIPLET/release/tex-fmt"

# run tex-fmt
$BINARY "$DIR/source"/* "$DIR/target"/*

# tex-fmt agrees with target files
for file in ../tests/source/*; do
    f=$(basename "$file")
    diff ../"tests/target/$f" "$DIR/source/$f" | diff-so-fancy
    diff ../"tests/target/$f" "$DIR/target/$f" | diff-so-fancy
done
