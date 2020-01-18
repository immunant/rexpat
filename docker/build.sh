#! /usr/bin/env bash

set -e

# do we have docker?
type -P docker >/dev/null || { 
    echo >&2 "docker not in path."; exit 1; 
}

# ... and the right version?
docker --version | grep -q "Docker version 17" && { 
    echo "Docker version too old. Please upgrade."; exit 1; }

REPO_NAME=immunant/libexpat-rs
DATE_TAG=$(date +'%Y%m%d')
SCRIPT_DIR="$(dirname "$0")"

 docker build \
    --tag "$REPO_NAME:$DATE_TAG" \
    --tag "$REPO_NAME:latest" \
    "$SCRIPT_DIR"
