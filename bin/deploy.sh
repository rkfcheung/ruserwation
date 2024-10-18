#!/usr/bin/env bash

echo "Updating latest codes ..."
git pull
if [ $? -ne 0 ]; then
    echo "ERROR: Failed to update codes"
    exit 1
fi

echo "Building ..."
cargo build --release
if [ $? -ne 0 ]; then
    echo "ERROR: Build failed"
    exit 1
fi

echo "Restarting the service ..."
sudo systemctl restart ruserwation
if [ $? -ne 0 ]; then
    echo "ERROR: Failed to restart the service"
    exit 1
fi

echo "Deployment successful!"
sudo systemctl status ruserwation
