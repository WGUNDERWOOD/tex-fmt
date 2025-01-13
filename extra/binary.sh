#!/usr/bin/env bash
echo "Testing binary"
DIR="$(mktemp -d)"
cp -r ../tests/* "$DIR"
cargo build --release

# run tex-fmt
for TESTDIR in "$DIR"/*; do
    ../target/release/tex-fmt -q "$TESTDIR/source"/*
done

# check tex-fmt agrees with target files
for TESTDIR in "$DIR"/*; do
    for file in "$TESTDIR/source"/*; do
        f=$(basename "$file")
        diff "$TESTDIR/target/$f" "$TESTDIR/source/$f" | diff-so-fancy
        diff "$TESTDIR/target/$f" "$TESTDIR/target/$f" | diff-so-fancy
    done
done
