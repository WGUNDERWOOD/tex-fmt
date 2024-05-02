DIR="$(mktemp -d)"
cp tests/phd_dissertation_in.tex $DIR

echo "Test file:"
for f in $DIR/*.tex; do
    echo -n "  $(basename $f), "
    echo -n "$(wc -l $f | cut --delimiter=" " --fields 1) lines, "
    echo "$(ls -sh $f | cut --delimiter=" " --fields 1)"
done

flamegraph -F 30000 -- ./target/release/tex-fmt $DIR/*.tex
