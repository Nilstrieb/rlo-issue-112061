#/usr/bin/env bash

rustc code.rs --crate-name bad -Zmir-enable-passes=-ConstProp -Copt-level=3
rustc code.rs --crate-name good -Zmir-enable-passes=+ConstProp -Copt-level=3

bad=$(./bad)
good=$(./good)

if [ "$good" != "$bad" ]; then
    echo "MISCOMPILATION"
else
    echo "no repro"
fi