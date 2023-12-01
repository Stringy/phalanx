#!/bin/bash

set -e

mkdir -p cmake-build

cmake -S bpf -B cmake-build -DCMAKE_BUILD_TYPE=Release
cmake --build cmake-build
