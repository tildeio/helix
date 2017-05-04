FROM bitnami/minideb:jessie

RUN apt-get update
RUN apt-get install curl -y --no-install-recommends
RUN apt-get install ca-certificates -y --no-install-recommends
RUN apt-get install wget tar -y --no-install-recommends
RUN apt-get install build-essential make -y --no-install-recommends
RUN apt-get install build-essential git -y --no-install-recommends

RUN adduser --disabled-password --gecos '' helix
USER helix

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH="/home/helix/.cargo/bin:${PATH}"

RUN rustup target add x86_64-unknown-linux-musl

RUN cd ~ && wget -O ruby-install-0.6.1.tar.gz https://github.com/postmodern/ruby-install/archive/v0.6.1.tar.gz
RUN cd ~ && tar -xzvf ruby-install-0.6.1.tar.gz

USER root

RUN cd /home/helix/ruby-install-0.6.1 && make install
RUN ruby-install ruby 2.4.1 --system

USER helix
ENV GEM_HOME=/home/helix/.gem
ENV GEM_PATH=/home/helix/.gem
ENV PATH="/home/helix/.gem/bin:${PATH}"
RUN gem install bundler rake --no-ri --no-rdoc

VOLUME "app"
WORKDIR "/app"
