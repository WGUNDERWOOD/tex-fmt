default: test doc clippy format shellcheck

all: default prof perf binary logo latex

alias b := build
alias d := doc
alias t := test
alias l := latex
alias c := clippy
alias f := format

build:
  @cargo build -r

test:
  @cargo test

doc:
  @cargo doc

testignored:
  @cargo test -- --ignored

clippy:
  @cargo clippy -r

format:
  @cargo fmt
  @alejandra -q .

latex:
  @cd extra && bash latex.sh

perf:
  @cd extra && bash perf.sh

prof:
  @cd extra && bash prof.sh

binary:
  @cd extra && bash binary.sh

upgrade:
  @cargo upgrade && cargo update

shellcheck:
  @shellcheck extra/*.sh

nix:
  @nix flake update

logo:
  @cd extra && python logo.py
  @cd extra && magick -background none logo.svg -resize 5000x5000 logo.png
  @cd extra && python card.py
  @cd extra && magick -background none card.svg -resize 1280x640\! card.png
  @cd extra && inkscape -w 2560 -h 1280 card.svg -o card.png
  @cd extra && rm -f logo.png card.svg
