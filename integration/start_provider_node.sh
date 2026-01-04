#!/bin/bash
set -x

for i in $(seq 0 "$1"); do
  (cd "node-deployer/services/upper-$i/yagna" && ./yagna-upper-"$i" service run) &
done