#!/bin/bash

if [ "$(uname)" = "Darwin" ]; then
    tmpdir="$(mktemp -d)"
    cp /bin/ls "$tmpdir"
    PATH=$tmpdir:$PATH
    trap 'rm -r "$tmpdir"' exit
fi

preload () {
    local library
    library=$1
    shift
    if [ "$(uname)" = "Darwin" ]; then
        DYLD_INSERT_LIBRARIES=target/debug/"$library".dylib "$@"
    else
        LD_PRELOAD=target/debug/"$library".so "$@"
    fi
}

set -ex
set -o pipefail

cd examples/readlinkspy
cargo update
cargo build
preload libreadlinkspy ls -l /dev/stdin | grep readlink

cd ../neverfree
cargo update
cargo build
