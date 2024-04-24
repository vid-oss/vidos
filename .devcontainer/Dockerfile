FROM ubuntu:24.04

RUN apt update && export DEBIAN_FRONTEND=noninteractive

RUN apt -y install build-essential git curl software-properties-common

# install rust
RUN curl --proto '=https' -sSf https://sh.rustup.rs/ | bash -s -- --default-toolchain=1.77.2 -y
ENV PATH="/root/.cargo/bin:${PATH}"

# install ffmpeg
RUN add-apt-repository ppa:ubuntuhandbook1/ffmpeg7
RUN apt update
RUN apt -y install ffmpeg
