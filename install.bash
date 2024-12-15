#!/bin/bash
apt-get update

apt-get install -y \
    build-essential \
    pkg-config \
    libudev-dev llvm libclang-dev \
    protobuf-compiler libssl-dev

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"

cargo install --git https://github.com/coral-xyz/anchor avm --force