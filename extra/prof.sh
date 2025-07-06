#!/usr/bin/env bash
echo "Making flamegraph profile"
DIR="$(mktemp -d)"
cp -r ../tests/* "$DIR"
CARGO_PROFILE_RELEASE_DEBUG=true cargo build --release
BIN="../target/release/tex-fmt"

mv "$DIR"/*/source/* "$DIR"
rm "$DIR"/*/target/*
find "$DIR" -name "*.toml" -delete
find "$DIR" -name "*.txt" -delete
find "$DIR"/* -empty -type d -delete
find "$DIR" -empty -type d -delete

echo -n "Test files: $(find "$DIR" | wc -l) files, "
echo -n "$(wc -l --total=only "$DIR"/*) lines, "
du -hs "$DIR" | cut -f 1
echo

flamegraph -F 10000 -- "$BIN" "$DIR"/*
