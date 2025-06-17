#!/bin/bash

set -e
cargo install cross || true

APPNAME="hyfetcher"

rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

echo "==> Building macOS (x86_64)..."
cargo build --release --target x86_64-apple-darwin

echo "==> Building macOS (arm64)..."
cargo build --release --target aarch64-apple-darwin

echo "==> Building Linux (x86_64)..."
cross build --release --target x86_64-unknown-linux-gnu

echo "==> Building Windows (x86_64)..."
cargo build --release --target x86_64-pc-windows-gnu

OUTDIR="release-bin"
mkdir -p $OUTDIR

cp target/x86_64-apple-darwin/release/$APPNAME      $OUTDIR/${APPNAME}-macos-amd64
cp target/aarch64-apple-darwin/release/$APPNAME     $OUTDIR/${APPNAME}-macos-arm64
cp target/x86_64-unknown-linux-gnu/release/$APPNAME $OUTDIR/${APPNAME}-linux-amd64
cp target/x86_64-pc-windows-gnu/release/${APPNAME}.exe $OUTDIR/${APPNAME}-windows-amd64.exe

# Compress outputs
cd $OUTDIR
tar -czvf ${APPNAME}-macos-amd64.tar.gz ${APPNAME}-macos-amd64
tar -czvf ${APPNAME}-macos-arm64.tar.gz ${APPNAME}-macos-arm64
tar -czvf ${APPNAME}-linux-amd64.tar.gz ${APPNAME}-linux-amd64
zip ${APPNAME}-windows-amd64.zip ${APPNAME}-windows-amd64.exe
cd ..

echo "==> All platform builds complete. Packages are in $OUTDIR directory."