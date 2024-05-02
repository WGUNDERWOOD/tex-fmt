DIR="$(mktemp -d)"
cp tests/*_in.* $DIR

echo "Test files:"
for f in $DIR/*; do
    echo -n "  $(basename $f), "
    echo -n "$(wc -l $f | cut --delimiter=" " --fields 1) lines, "
    echo "$(ls -sh $f | cut --delimiter=" " --fields 1)"
done

echo
echo -n "Total: $(ls -l $DIR/* | wc -l) files, "
echo -n "$(wc -l --total=only $DIR/*) lines, "
echo "$(ls -lh $DIR | head -n 1 | cut --delimiter=" " --fields 2)"
echo

hyperfine --warmup 2 \
    -n "tex-fmt" "./target/release/tex-fmt $DIR/*" \
    -n "latexindent" "latexindent $DIR/*" \
    -n "latexindent -m" "latexindent -m $DIR/*"
