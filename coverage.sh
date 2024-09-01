#!/bin/bash


p=$1
m=$2

cargo clean

find ./coverage_prof -type f ! -name ".gitkeep" -exec rm -f {} +
find ./coverage -type f ! -name ".gitkeep" -exec rm -f {} +
echo "Start coverage"

toolchain=$(rustup default)

if [[ "$toolchain" == "nightly-x86_64-pc-windows-msvc (default)" ]]; then
    export RUSTFLAGS="-Cinstrument-coverage"
elif [[ "$toolchain" == "my-nightly (default)" ]]; then
    export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
else
    echo "Err:Unexpected toolchain."
    exit 1
fi

if [[ -n "$p" ]]; then
    echo "Package :$p"
fi

if [[ -n "$m" ]]; then
    echo "Module :$m"
fi

export CARGO_INCREMENTAL=0
export LLVM_PROFILE_FILE="../coverage_prof/traq-bot-http-rs-%p-%m.profraw"

if [[ -n "$p" && -n "$m" ]]; then
    cmd="cargo test $m -p $p"
    echo "Command :$cmd"
    eval $cmd
elif [[ -n "$p" ]]; then
    cargo test -p "$p"
else
    cargo test
fi

grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./coverage/

unset CARGO_INCREMENTAL
unset RUSTFLAGS
unset LLVM_PROFILE_FILE

xdg-open "./coverage/html/index.html" 2>/dev/null || open "./coverage/html/index.html"