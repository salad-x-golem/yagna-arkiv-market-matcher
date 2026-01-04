#!/bin/bash
set -x

for i in {0..9}; do
  (cd node-deployer/services/upper-"$i"/yagna && ./ya-provider run) &
done
