default: test clippy format shellcheck

all: default prof perf latex logo

alias b := build
alias t := test
alias l := latex
alias c := clippy
alias f := format

build:
  @cargo build -r

test:
  @cargo test -r

testignored:
  @cargo test -r -- --ignored

clippy:
  @cargo clippy -r

format:
  @cargo fmt

latex:
  @cd extra && bash latex.sh

perf:
  @cd extra && bash perf.sh

prof:
  @cd extra && bash prof.sh

shellcheck:
  @shellcheck extra/*.sh

logo:
  @cd extra && python logo.py
  @cd extra && magick -background none logo.svg -resize 5000x5000 logo.png
  @cd extra && python card.py
  @cd extra && magick -background none card.svg -resize 1280x640\! card.png
  @cd extra && inkscape -w 2560 -h 1280 card.svg -o card.png
  @cd extra && rm -f logo.png card.svg
