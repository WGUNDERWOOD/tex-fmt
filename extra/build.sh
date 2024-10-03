#!/usr/bin/env bash
DIR="$(mktemp -d)"
cp -r ../tests/* "$DIR"
TRIPLET=$(gcc -dumpmachine)
BINARY="../target/$TRIPLET/release/tex-fmt"
cargo pgo build

hyperfine --min-runs 10 \
    --prepare "cp -r ../tests/* $DIR" \
    "$BINARY $DIR/source/* $DIR/target/*"

cargo pgo optimize
