default: test doc clippy format shellcheck shellinstall wasm

all: default prof binary logo perf latex

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

shellinstall:
  @cargo build -r --features shellinstall

testignored:
  @cargo test -- --ignored

clippy:
  @cargo clippy -r && cargo shear

format:
  @cargo fmt
  @alejandra -q .

latex:
  @cd extra && bash latex.sh

wasm:
  @mkdir -p web/pkg
  @cargo build -r --lib --target wasm32-unknown-unknown
  @wasm-bindgen --target web --out-dir web/pkg \
      target/wasm32-unknown-unknown/release/tex_fmt.wasm
  @cd web/pkg && wasm-opt -Oz -o tex_fmt_bg.wasm tex_fmt_bg.wasm

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

ctan:
  @cp ctan/README.md ctan/tex-fmt
  @pandoc -o ctan/tex-fmt/tex-fmt.pdf ctan/README.md
  @cd ctan && tar -czf tex-fmt.tar.gz tex-fmt
  @cd ctan && ctan-o-mat tex-fmt.pkg

nix:
  @nix flake update

todo:
  @rg -g '!justfile' todo

logo:
  @cd extra && python logo.py
  @cd extra && magick -background none logo.svg -resize 5000x5000 logo.png
  @cd extra && python card.py
  @cd extra && magick -background none card.svg -resize 1280x640\! card.png
  @cd extra && inkscape -w 2560 -h 1280 card.svg -o card.png
  @cd extra && rm -f logo.png card.svg
