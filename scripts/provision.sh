#!/bin/bash

sudo apt-get install git -y
sudo apt-get install mingw-w64 -y

curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y
source $HOME/.cargo/env

\curl -sSL https://get.rvm.io | bash -s stable --ruby
source $HOME/.rvm/scripts/rvm

gem install bundler
