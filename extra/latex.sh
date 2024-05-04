DIR="$(mktemp -d)"
cp ../tests/*_in.tex $DIR
cp ../tests/*_out.tex $DIR
cp *.cls $DIR
cd $DIR
echo $DIR

for file in *_in.tex; do
    f=$(basename $file _in.tex)
    echo $f
    latexmk -pdf ${f}_in.tex
    latexmk -pdf ${f}_out.tex
    pdftotext ${f}_in.pdf
    pdftotext ${f}_out.pdf
    diff -u ${f}_in.txt ${f}_out.txt | diff-so-fancy
done
