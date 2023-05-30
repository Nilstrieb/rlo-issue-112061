#!/usr/bin/env bash

rustc code.rs --crate-name ll -Zmir-enable-passes=-ConstProp --emit llvm-ir -Cno-prepopulate-passes --crate-type=lib

clang ll.ll helper.c -O1 -o good
clang ll.ll helper.c -O2 -o bad

bad=$(./bad)
good=$(./good)

if [ "$good" != "$bad" ]; then
    echo "MISCOMPILATION"
else
    echo "no repro"
fi