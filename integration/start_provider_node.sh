#!/bin/bash
set -x

for i in {0..9}; do
  (cd node-deployer/services/upper-"$i"/yagna && ./yagna-upper-"$i" service run) &
done
