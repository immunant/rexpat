#! /usr/bin/env bash

set -e

# do we have docker?
type -P docker >/dev/null || { 
    echo >&2 "docker not in path."; exit 1; 
}

# ... and the right version?
docker --version | grep -q "Docker version 17" && { 
    echo "Docker version too old. Please upgrade."; exit 1; }

REPO_NAME=immunant/rexpat
DATE_TAG=$(date +'%Y%m%d')
SCRIPT_DIR="$(dirname "$0")"

# pull the rust version out of ../rust-toolchain to keep things synched
RUST_TOOLCHAIN_FILE="$SCRIPT_DIR/../rust-toolchain"
RUST_VER=$(cat $RUST_TOOLCHAIN_FILE | tr -d '\n')

 docker build \
    --tag "$REPO_NAME:$DATE_TAG" \
    --tag "$REPO_NAME:latest" \
    --build-arg RUST_VER=$RUST_VER \
    "$SCRIPT_DIR"
