#!/usr/bin/env bash
echo "Making flamegraph profile"
DIR="$(mktemp -d)"
cp -r ../tests/* "$DIR"
CARGO_PROFILE_RELEASE_DEBUG=true cargo build --release
BIN="../target/release/tex-fmt"

echo
echo -n "Test files: $(find "$DIR"/*/* | wc -l) files, "
echo -n "$(wc -l --total=only "$DIR"/source/* "$DIR"/target/*) lines, "
du -hs "$DIR" | cut -f 1
echo

flamegraph -F 10000 -- "$BIN" "$DIR/source/"* "$DIR/target/"*
