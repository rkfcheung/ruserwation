#!/usr/bin/env bash

sudo systemctl daemon-reload
sudo systemctl enable ruserwation
sudo systemctl start ruserwation
sudo systemctl status ruserwation
