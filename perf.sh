DIR="$(mktemp -d)"
cp tests/*_in.tex $DIR

echo "Test files:"
for f in $DIR/*.tex; do
    echo -n "  $(basename $f), "
    echo -n "$(wc -l $f | cut --delimiter=" " --fields 1) lines, "
    echo "$(ls -sh $f | cut --delimiter=" " --fields 1)"
done
echo -n "Total: $(wc -l --total=only $DIR/*.tex) lines, "
echo "$(ls -lh $DIR | head -n 1 | cut --delimiter=" " --fields 2)"
echo

hyperfine --warmup 2 \
    -n "tex-fmt" "tex-fmt $DIR/*.tex" \
    -n "latexindent" "latexindent $DIR/*.tex" \
    -n "latexindent -m" "latexindent -m $DIR/*.tex"
