FROM ubuntu:24.04

RUN apt update && export DEBIAN_FRONTEND=noninteractive

RUN apt -y install software-properties-common

RUN add-apt-repository ppa:ubuntuhandbook1/ffmpeg7 && \
    add-apt-repository ppa:apt-fast/stable

RUN apt update && apt -y install apt-fast

RUN echo debconf apt-fast/maxdownloads string 16 | debconf-set-selections && \
    echo debconf apt-fast/dlflag boolean true | debconf-set-selections && \
    echo debconf apt-fast/aptmanager string apt-get | debconf-set-selections

RUN apt-fast -y install build-essential git curl software-properties-common ffmpeg

# install rust
RUN curl --proto '=https' -sSf https://sh.rustup.rs/ | bash -s -- --default-toolchain=1.77.2 -y
ENV PATH="/root/.cargo/bin:${PATH}"
