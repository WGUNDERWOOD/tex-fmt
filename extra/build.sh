#!/usr/bin/env bash
DIR="$(mktemp -d)"
cp -r ../tests/* "$DIR"
TRIPLET=$(gcc -dumpmachine)
BINARY="../target/$TRIPLET/release/tex-fmt"
cargo pgo build

hyperfine --warmup 10 \
    --min-runs 50 \
    --command-name "tex-fmt" \
    --prepare "cp -r ../tests/* $DIR" \
    "$BINARY $DIR/source/* $DIR/target/*"

cargo pgo optimize
