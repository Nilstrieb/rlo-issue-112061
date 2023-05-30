#!/usr/bin/env bash

set -eu

# rustc code.rs --crate-name ll -Zmir-enable-passes=-ConstProp --emit llvm-ir -Cno-prepopulate-passes --crate-type=lib

clang $1 helper.c -O1 -o good
clang $1 helper.c -O2 -o bad

bad=$(./bad)
good=$(./good)

if [ "$good" != "$bad" ]; then
    echo "MISCOMPILATION"
else
    echo "no repro"
    exit 1
fi