#!/bin/bash

set -e
set -x

pushd ../../ruby
bundle install
bundle exec rake native_lib_files
popd

cp ../../ruby/windows_build/helix-runtime-*.i386.lib .
cp ../../ruby/windows_build/helix-runtime-*.x86_64.lib .
