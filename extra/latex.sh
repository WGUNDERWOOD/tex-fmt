echo "Checking latex PDFs agree"
DIR="$(mktemp -d)"
cp ../tests/*_in.tex $DIR
cp ../tests/*_out.tex $DIR
cp ../tests/pu_thesis_in.cls $DIR/puthesis.cls
cp ../tests/ociam_thesis_in.cls $DIR/ociamthesis.cls
cp ../tests/wgu_cv_in.cls $DIR/wgu-cv.cls
rm $DIR/empty_in.tex $DIR/empty_out.tex
cd $DIR
echo $DIR

echo

for file in *_in.tex; do
    f=$(basename $file _in.tex)
    echo "Running latex for $f"
    latexmk -pdf -quiet -rc-report- ${f}_in.tex >/dev/null 2>&1
    latexmk -pdf -quiet -rc-report- ${f}_out.tex >/dev/null 2>&1
    pdftotext -q ${f}_in.pdf >/dev/null 2>&1
    pdftotext -q ${f}_out.pdf >/dev/null 2>&1
done

echo

for file in *_in.tex; do
    f=$(basename $file _in.tex)
    echo "Checking PDF for $f"
    diff -u ${f}_in.txt ${f}_out.txt | diff-so-fancy
done
