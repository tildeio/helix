#!/bin/bash

pushd ../../ruby
bundle install
bundle exec rake native_lib_file
popd

cp ../../ruby/windows_build/helix-runtime*.lib .
