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

# /home/docker needs to be accessible by other users because we always run the
# cargo binary from /home/docker.cargo/bin
RUN chmod 755 /home/docker

# required to install package dependencies.
# NOTE: Ubuntu 18.04 ships with Python3.6 by defa ultwhich triggers a UnicodeException
# in the W3C testsuite so we update the default python3 to Python3.7.
# NOTE: we reduce the size of the image by deleting 
RUN sed -Ei 's/^# deb-src /deb-src /' /etc/apt/sources.list
RUN apt-get update -qq && \    
    apt-get install -qq   \
    bear                  \
    build-essential       \
    curl                  \
    ed                    \
	gdb                   \
    git                   \
    python3.7-minimal     \
    libcap-dev            \
    libtool-bin           \
    libsqlite3-dev     && \
    apt-get build-dep -qq squid php && \
    update-alternatives --install /usr/bin/python3 python3 /usr/bin/python3.7 100 && \
    rm -rf /var/lib/apt/lists/*

ARG RUST_VER=nightly-2020-07-27
USER docker
WORKDIR ${HOME}
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs |  \
    sh -s -- -y --default-toolchain $RUST_VER
RUN echo "source ~/.cargo/env" >> .bashrc

ENV TMP=/tmp
ENV XML_TEST_TMP=${TMP}/libexpat
ENV XML_TEST_URL=http://www.w3.org/XML/Test/xmlts20020606.tar
ENV XML_ARCHIVE=${XML_TEST_TMP}/xmlts20020606.tar
RUN mkdir -p "${XML_TEST_TMP}" && \
    curl "${XML_TEST_URL}" --silent --output "${XML_ARCHIVE}" && \
    tar -C "${XML_TEST_TMP}"  -xf "${XML_ARCHIVE}" && \
    rm "${XML_ARCHIVE}"

ENV XML_TEST_TMP=${TMP}/libexpat/xml-test-suite-2013
ENV XML_TEST_URL=https://www.w3.org/XML/Test/xmlts20130923.tar.gz
ENV XML_ARCHIVE=${XML_TEST_TMP}/xmlts20130923.tar.gz
RUN mkdir -p "${XML_TEST_TMP}" && \
    curl "${XML_TEST_URL}" --silent --output "${XML_ARCHIVE}" && \
    tar -C "${XML_TEST_TMP}"  -xf "${XML_ARCHIVE}" && \
    rm "${XML_ARCHIVE}"    

ENV PYTHON381_TMP=${TMP}/Python-3.8.1
ENV PYTHON381_URL=https://www.python.org/ftp/python/3.8.1/Python-3.8.1.tgz
ENV PYTHON381_ARCHIVE=${TMP}/Python-3.8.1.tgz
RUN curl -L ${PYTHON381_URL} --silent --output "${PYTHON381_ARCHIVE}" && \
    tar -C /tmp -xf "${PYTHON381_ARCHIVE}" && \
    rm "${PYTHON381_ARCHIVE}"
RUN cd ${PYTHON381_TMP} && ./configure --quiet --with-system-expat && \
    make -j`nproc --all` 1> build.stdout.log 2> build.stderr.log

ENV SQUID_TMP=${TMP}/squid-4.10
ENV SQUID_URL=http://www.squid-cache.org/Versions/v4/squid-4.10.tar.gz
ENV SQUID_ARCHIVE=${TMP}/squid-4.10.tar.gz
RUN curl -L ${SQUID_URL} --silent --output "${SQUID_ARCHIVE}" && \
    tar -C /tmp -xf "${SQUID_ARCHIVE}" && \
    rm "${SQUID_ARCHIVE}"
RUN cd ${SQUID_TMP} && ./configure --quiet --enable-esi --with-expat 
# &&   make -j`nproc --all` check 1> build.stdout.log 2> build.stderr.log

# only one binary seems to actually use libexpat, so we skip git
# ENV GIT_TMP=${TMP}/git-2.25.0  
# ENV GIT_URL=https://github.com/git/git/archive/v2.25.0.tar.gz
# ENV GIT_ARCHIVE=${TMP}/git-2.25.0.tar.gz
# RUN curl -L ${GIT_URL} --silent --output "${GIT_ARCHIVE}" && \
#     tar -C /tmp -xf "${GIT_ARCHIVE}" && \ 
#     rm "${GIT_ARCHIVE}"
# RUN cd ${GIT_TMP} && autoconf && ./configure --quiet && \
#    make -j`nproc --all` 1> build.stdout.log 2> build.stderr.log

ENV PHP_TMP=${TMP}/php-7.4.2
ENV PHP_URL=https://www.php.net/distributions/php-7.4.2.tar.bz2
ENV PHP_ARCHIVE=${TMP}/php-7.4.2.tar.bz2
RUN curl -L ${PHP_URL} --silent --output "${PHP_ARCHIVE}" && \
    tar -C /tmp -xf "${PHP_ARCHIVE}" && \
    rm "${PHP_ARCHIVE}"
RUN cd ${PHP_TMP} && ./configure --quiet --with-expat && \ 
    make -j`nproc --all` 1> build.stdout.log 2> build.stderr.log

ENV PATH="/home/${USER}/.cargo/bin:${PATH}"
ENV CARGO_HOME="/tmp/.cargo"
ENV RUSTUP_HOME="/home/${USER}/.rustup"

# Need to switch back to root for Teamcity and Azure DevOps compatibility
USER root

RUN chmod -R 777 /tmp
