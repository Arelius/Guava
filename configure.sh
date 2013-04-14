#!/bin/sh

RUSTTAR=rust-0.6.tar.gz
INSTDIR=$(pwd)/bin/rust

mkdir dep
mkdir bin
cd dep
curl http://static.rust-lang.org/dist/$RUSTTAR -o $RUSTTAR
tar -xzf $RUSTTAR
cd rust-0.6
./configure --prefix=$INSTDIR
make
make install

cd ../..

