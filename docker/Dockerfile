ARG BASE_IMAGE=ubuntu:bionic
FROM ${BASE_IMAGE}
LABEL maintainer="perl@immunant.com"

ARG USER=docker
ARG UID=1000
ARG GID=1000
ENV HOME=/home/$USER
ENV DEBIAN_FRONTEND=noninteractive

USER root
RUN groupadd -f -g $GID $USER
RUN useradd --create-home -u $UID -g $GID --create-home --shell /bin/bash $USER

RUN apt-get update -qq && \
    apt-get install -qq \
    curl git clang python3-minimal && \
    rm -rf /tar/lib/apt/lists/*

ARG RUST_VER=nightly-2019-12-17
USER docker
WORKDIR ${HOME}
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs |  \
    sh -s -- -y --default-toolchain $RUST_VER
RUN echo "source ~/.cargo/env" >> .bashrc

ENV XML_TEST_TMP=/tmp/libexpat
ENV XML_TEST_SUITE=/tmp/libexpat/xml-test-suite
ENV XML_ARCHIVE=${XML_TEST_TMP}/xmlts20020606.tar
RUN mkdir -p "${XML_TEST_TMP}" && \
    curl http://www.w3.org/XML/Test/xmlts20020606.tar --silent --output "${XML_ARCHIVE}" && \
    tar -C "${XML_TEST_TMP}"  -xf "${XML_ARCHIVE}" 
