echo "Checking latex PDFs agree"
DIR="$(mktemp -d)"
cp -r ../tests/* $DIR
cd $DIR
echo $DIR

echo

for file in ./in/*.tex; do
    f=$(basename $file .tex)
    echo "Running latex for $f.tex"
    (cd ./in && latexmk -pdf -quiet -rc-report- $f.tex >/dev/null 2>&1)
    (cd ./out && latexmk -pdf -quiet -rc-report- $f.tex >/dev/null 2>&1)
    (cd ./in && pdftotext -q $f.pdf >/dev/null 2>&1)
    (cd ./out && pdftotext -q $f.pdf >/dev/null 2>&1)
done

echo

for file in ./in/*.tex; do
    f=$(basename $file .tex)
    echo "Checking PDF for $f.tex"
    diff -u ./in/$f.txt ./out/$f.txt | diff-so-fancy
done
