#/usr/bin/env bash

rustc code.rs --crate-name bad -Zmir-enable-passes=-ConstProp
rustc code.rs --crate-name good -Zmir-enable-passes=+ConstProp

bad=$(./bad)
good=$(./good)

if [ good != bad ]; then
    echo "MISCOMPILATION"
else
    echo "no repro"
fi