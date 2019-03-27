#!/bin/bash
cargo build --manifest-path utility/Cargo.toml
cd docker
docker build -t pwndock --build-arg uid=$(id -u) --build-arg gid=$(id -g) .
