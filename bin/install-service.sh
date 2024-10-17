#!/usr/bin/env bash

THIS_DIR=$(cd $(dirname $0); pwd)

sudo cp ${THIS_DIR}/../etc/services/ruserwation.service /etc/systemd/system/ruserwation.service

sudo systemctl daemon-reload
sudo systemctl enable ruserwation
sudo systemctl start ruserwation
sudo systemctl status ruserwation
