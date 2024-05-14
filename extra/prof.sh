echo "Making flamegraph profile"
DIR="$(mktemp -d)"
cp ../tests/phd_dissertation_in.tex $DIR
cargo build --release

echo "Writing large test file"
for i in {1..10}; do
    cat $DIR/phd_dissertation_in.tex >> $DIR/large.tex
    echo "\n\n\n" >> $DIR/large.tex
done

rm $DIR/phd_dissertation_in.tex

echo "Test file:"
for f in $DIR/*.tex; do
    echo -n "  $(basename $f), "
    echo -n "$(wc -l $f | cut --delimiter=" " --fields 1) lines, "
    echo "$(ls -sh $f | cut --delimiter=" " --fields 1)"
done

flamegraph -F 10000 -- ../target/release/tex-fmt $DIR/*.tex
