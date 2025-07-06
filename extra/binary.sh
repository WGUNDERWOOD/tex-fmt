#!/usr/bin/env bash
echo "Testing binary"

# tempdir for unmodified test files
DIR_ORIG="$(mktemp -d)"
cp -r ../tests/* "$DIR_ORIG"

# tempdir for formatted test files
DIR_TEST="$(mktemp -d)"
cp -r ../tests/* "$DIR_TEST"

# build binary
cargo build --release
BIN=$(realpath "../target/release/tex-fmt")

# run tex-fmt in DIR_TEST
for TESTDIR in "$DIR_TEST"/*; do
    FLAGS="-q"
    if [ -f "$TESTDIR/tex-fmt.toml" ]; then
        FLAGS="$FLAGS --config $TESTDIR/tex-fmt.toml"
    else
        FLAGS="$FLAGS --noconfig"
    fi
    if [ -f "$TESTDIR/cli.txt" ]; then
        FLAGS+=" $(paste -sd' ' "$TESTDIR/cli.txt")"
    fi
    (cd "$TESTDIR" && eval "$BIN $FLAGS" "$TESTDIR/source"/*)
    (cd "$TESTDIR" && eval "$BIN $FLAGS" "$TESTDIR/target"/*)
done

# check tex-fmt agrees with target files
for TESTDIR in "$DIR_TEST"/*; do
    DIRNAME=$(basename "$TESTDIR")
    for file in "$TESTDIR/source"/*; do
        f=$(basename "$file")
        diff "$DIR_ORIG/$DIRNAME/target/$f" "$TESTDIR/target/$f" | diff-so-fancy
        diff "$DIR_ORIG/$DIRNAME/target/$f" "$TESTDIR/source/$f" | diff-so-fancy
    done
done

# if both config and cli exist, run tex-fmt again in DIR_TEST
for TESTDIR in "$DIR_TEST"/*; do
    DIRNAME=$(basename "$TESTDIR")
    if [ -f "$TESTDIR/tex-fmt.toml" ] && [ -f "$TESTDIR/cli.txt" ]; then
        FLAGS="-q --noconfig"
        FLAGS+=" $(paste -sd' ' "$TESTDIR/cli.txt")"
        cp -r "$DIR_ORIG/$DIRNAME"/* "$TESTDIR"
        (cd "$TESTDIR" && eval "$BIN $FLAGS" "$TESTDIR/source"/*)
        (cd "$TESTDIR" && eval "$BIN $FLAGS" "$TESTDIR/target"/*)
    fi
done

# check tex-fmt agrees with target files
for TESTDIR in "$DIR_TEST"/*; do
    DIRNAME=$(basename "$TESTDIR")
    for file in "$TESTDIR/source"/*; do
        f=$(basename "$file")
        diff "$DIR_ORIG/$DIRNAME/target/$f" "$TESTDIR/target/$f" | diff-so-fancy
        diff "$DIR_ORIG/$DIRNAME/target/$f" "$TESTDIR/source/$f" | diff-so-fancy
    done
done
