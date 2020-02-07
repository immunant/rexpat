#! /usr/bin/env bash

set -e

# do we have docker?
type -P docker >/dev/null || { 
    echo >&2 "docker not in path."; exit 1; 
}

USER=docker
SCRIPT_PATH="$( cd "$(dirname "$0")" ; pwd -P )"
REXPAT_ROOT="$(dirname "$SCRIPT_PATH")"
docker run -it --user $USER \
    --rm --name rexpat_ephemeral \
    --hostname rexpat_ephemeral \
    --workdir "/home/$USER/rexpat" \
    -v "$REXPAT_ROOT":"/home/$USER/rexpat" \
    immunant/rexpat:latest

