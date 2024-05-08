default: test latex clippy format prof

all: default perf

alias t := test
alias l := latex
alias c := clippy
alias f := format

test:
  @cargo test

latex:
  @cd extra && bash latex.sh

perf:
  @cd extra && bash perf.sh

prof:
  @cd extra && bash prof.sh

clippy:
  @cargo clippy

format:
  @cargo fmt
