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
  pkill -9 yp-"${MACHINE_PROV}"-"$i" || true
  pkill -9 yagna-"${MACHINE_PROV}"-"$i" || true
  pkill -9 vanity-lower-"$i" || true
  pkill -9 yagna-lower-"$i" || true
done

pkill -9 router-geode || true
pkill -9 yagna-offer-server || true