#!/usr/bin/env bash
echo "Testing binary"
DIR="$(mktemp -d)"
cp -r ../tests/* "$DIR"
cargo build --release
BIN=$(realpath "../target/release/tex-fmt")

# run tex-fmt
for TESTDIR in "$DIR"/*; do
    (cd "$TESTDIR" && "$BIN" -q "$TESTDIR/source"/*)
done

# check tex-fmt agrees with target files
for TESTDIR in "$DIR"/*; do
    for file in "$TESTDIR/source"/*; do
        f=$(basename "$file")
        diff "$TESTDIR/target/$f" "$TESTDIR/source/$f" | diff-so-fancy
        diff "$TESTDIR/target/$f" "$TESTDIR/target/$f" | diff-so-fancy
    done
done
