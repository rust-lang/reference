#!/bin/sh

set -e

if [ ! -f book.toml ]
then
    echo "Run command in root directory with book.toml"
    exit 1
fi

rm -rf tests/linkcheck tests/linkchecker

mkdir tests/linkchecker
curl -o tests/linkchecker/Cargo.toml \
    https://raw.githubusercontent.com/rust-lang/rust/master/src/tools/linkchecker/Cargo.toml
curl -o tests/linkchecker/main.rs \
    https://raw.githubusercontent.com/rust-lang/rust/master/src/tools/linkchecker/main.rs

mdbook build

cp -R $(rustc --print sysroot)/share/doc/rust/html tests/linkcheck
rm -rf tests/linkcheck/reference
cp -R book tests/linkcheck/reference

cargo run --manifest-path=tests/linkchecker/Cargo.toml -- tests/linkcheck/reference

rm -rf tests/linkcheck tests/linkchecker
echo "Linkcheck completed successfully!"
