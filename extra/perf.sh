echo "Getting performance metrics"
DIR="$(mktemp -d)"
cp -r ../tests/* $DIR
cargo build --release

calc(){ awk "BEGIN { print "$*" }"; }

echo
echo -n "Test files: $(ls -l $DIR/in/* $DIR/out/* | wc -l) files, "
echo -n "$(wc -l --total=only $DIR/in/* $DIR/out/*) lines, "
echo "$(du -hs $DIR | cut -f 1)"
echo

# tex-fmt
TEXFMTFILE="hyperfine-tex-fmt.csv"
hyperfine --warmup 10 \
    --export-csv $TEXFMTFILE \
    --command-name "tex-fmt" \
    --prepare "cp -r ../tests/* $DIR" \
    "../target/release/tex-fmt $DIR/in/* $DIR/out/*"

# latexindent
LATEXINDENTFILE="hyperfine-latexindent.csv"
hyperfine --warmup 0 \
    --export-csv $LATEXINDENTFILE \
    --runs 1 \
    --command-name "latexindent" \
    --prepare "cp -r ../tests/* $DIR" \
    "latexindent $DIR/in/* $DIR/out/*"

# latexindent -m
LATEXINDENTMFILE="hyperfine-latexindent-m.csv"
hyperfine --warmup 0 \
    --export-csv $LATEXINDENTMFILE \
    --runs 1 \
    --command-name "latexindent -m" \
    --prepare "cp -r ../tests/* $DIR" \
    "latexindent -m $DIR/in/* $DIR/out/*"

# print results
TEXFMT=$(cat $TEXFMTFILE | tail -n 1 | cut -d "," -f 2)
echo "tex-fmt: ${TEXFMT}s"

LATEXINDENT=$(cat $LATEXINDENTFILE | tail -n 1 | cut -d "," -f 2)
LATEXINDENTTIMES=$(calc $LATEXINDENT/$TEXFMT)
echo "latexindent: ${LATEXINDENT}s, x$LATEXINDENTTIMES"

LATEXINDENTM=$(cat $LATEXINDENTMFILE | tail -n 1 | cut -d "," -f 2)
LATEXINDENTMTIMES=$(calc $LATEXINDENTM/$TEXFMT)
echo "latexindent -m: ${LATEXINDENTM}s, x$LATEXINDENTMTIMES"
