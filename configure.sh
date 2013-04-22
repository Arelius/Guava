#!/bin/sh

RUSTTAR=rust-0.6.tar.gz
INSTDIR=$(pwd)/dep

mkdir dep
mkdir tmp
cd tmp

# Install Rust
curl http://static.rust-lang.org/dist/$RUSTTAR -o $RUSTTAR
tar -xzf $RUSTTAR
cd rust-0.6
./configure --prefix=$INSTDIR
make
make install
cd ..

# Install SDL
hg clone http://hg.libsdl.org/SDL/
mkdir SDL-Build
cd SDL-Build
../SDL/configure --prefix=$INSTDIR/SDL
make
make install

cd ..

