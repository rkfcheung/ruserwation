#!/usr/bin/env bash

THIS_DIR=$(cd $(dirname $0); pwd)

echo "Updating latest codes ..."
git pull

echo "Building ..."
cargo build --release

echo "Restarting the service ..."
sudo systemctl restart ruserwation
sudo systemctl status ruserwation
