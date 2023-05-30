#!/usr/bin/env bash

set -eu

# rustc code.rs --crate-name ll -Zmir-enable-passes=-ConstProp --emit llvm-ir -Cno-prepopulate-passes --crate-type=lib

d=$(mktemp -d)

clang $1 helper.c -O1 -o "$d/good"
clang $1 helper.c -O2 -o "$d/bad"

bad=$("$d/bad")
good=$("$d/good")

if [ "$good" != "$bad" ]; then
    echo "MISCOMPILATION"
else
    echo "no repro"
    exit 1
fi