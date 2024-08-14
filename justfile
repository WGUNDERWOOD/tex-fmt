default: test clippy format shellcheck

all: default prof perf latex

alias t := test
alias l := latex
alias c := clippy
alias f := format

test:
  @cargo test -r

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
