#!/bin/bash

set -eoux pipefail

MIRIFLAGS="\
  --remap-path-prefix=$(rustc --print=sysroot)/lib/rustlib/src/rust/library/= \
  -Zkmiri-toml=$PWD/kmiri.toml" \
MIRI_SYSROOT="$(rustc --print sysroot)" \
  cargo miri run --target riscv64imac-unknown-none-elf
