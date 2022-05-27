#!/bin/bash

echo "Get version"
version="$(sed -n 's/^version = "\(.*\)"/\1/p' ../../Cargo.toml | head -n1)"
echo "Version: $version"

echo "Creating stable release PKGBUILD-stable"
echo "Replacing version in PKGBUILD-stable"
sed -i "s/pkgver=[1-9]\+[0-9]*\(\.[0-9]\+\)\{2\}/pkgver=$version/" PKGBUILD-stable

echo "Replacing hash in PKGBUILD-stable"

echo "Downloading release and creating hash"
hash="$(curl -sL https://github.com/finefindus/artem/archive/v$version.tar.gz | sha256sum | cut -d ' ' -f 1)"

sed -i "s/sha256sums=('.*')/sha256sums=('$hash')/" PKGBUILD-stable


echo "Creating bin release PKGBUILD-bin"
echo "Replacing version in PKGBUILD-bin"
sed -i "s/pkgver=[1-9]\+[0-9]*\(\.[0-9]\+\)\{2\}/pkgver=$version/" PKGBUILD-bin

echo "Replacing hash in PKGBUILD-bin"

echo "Downloading release and creating hash"
hashbin="$(curl -sL https://github.com/FineFindus/artem/releases/download/v$version/artem-v$version-x86_64-unknown-linux-gnu.tar.gz | sha256sum | cut -d ' ' -f 1)"

sed -i "s/sha256sums=('.*')/sha256sums=('$hashbin')/" PKGBUILD-bin
