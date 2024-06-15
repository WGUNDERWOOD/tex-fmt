echo "Checking latex PDFs agree"
DIR="$(mktemp -d)"
cp -r ../tests/* $DIR
echo $DIR

# TODO remove block
rm $DIR/in/*
cp -r ../tests/in/higher* $DIR/in/
cp -r ../tests/in/cam* $DIR/in/
cp -r ../tests/in/quiver* $DIR/in/

cd $DIR

echo

for file in ./in/*.tex; do
    f=$(basename $file .tex)
    echo "Running latex for $f.tex"
    (cd ./in && latexmk -pdflua $f.tex )#>/dev/null 2>&1)
    (cd ./out && latexmk -pdflua $f.tex )#>/dev/null 2>&1)
    (cd ./in && pdftotext -q $f.pdf >/dev/null 2>&1)
    (cd ./out && pdftotext -q $f.pdf >/dev/null 2>&1)
done

echo

for file in ./in/*.tex; do
    f=$(basename $file .tex)
    echo "Checking PDF for $f.tex"
    diff -u ./in/$f.txt ./out/$f.txt | diff-so-fancy
done

echo $DIR
