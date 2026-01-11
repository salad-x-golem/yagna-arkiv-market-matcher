#!/bin/bash

if [ -z "$1" ]; then
  echo "usage: $0 N"
  exit 1
fi

end=$(( $1 - 1 ))
if [ "$end" -lt 0 ]; then
  echo "no nodes to start"
  exit 0
fi

set -x

MACHINE_PROV=${MACHINE_PROV:-"geode"}

for i in $(seq 0 "$end"); do
  (cd "node-deployer/services/${MACHINE_PROV}-$i/yagna" && ./yagna-"${MACHINE_PROV}"-"$i" service run >/dev/null 2>&1) &
  sleep 0.1
done