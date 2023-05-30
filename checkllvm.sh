#!/usr/bin/env bash

rustc code.rs --crate-name ll -Zmir-enable-passes=-ConstProp --emit llvm-ir -Cno-prepopulate-passes --crate-type=lib

clang ll.ll helper.c

./a.out
