#!/usr/bin/env bash

set -o errexit
set -o nounset
set -o pipefail

build() {
    cargo run --manifest-path ./generator/Cargo.toml relaxed $PWD/build
}

detect_changes() {
    inotifywait generator/src static/ blog/ --event modify,move,create,delete,attrib
}

build
while detect_changes ; do 
  build
done &
generator_pid=$!

(cd ./build/build && python3 -m http.server) &
server_pid=$!

terminate() {
  kill $generator_pid
  kill $server_pid
}

trap terminate EXIT

wait $generator_pid $server_pid
wait 



