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
  tar -cf provider-yagnadir-"$i".tar.gz node-deployer/services/"${MACHINE_PROV}"-"$i"/yagna/yagnadir node-deployer/services/"${MACHINE_PROV}"-"$i"/yagna/.env
  tar -cf provider-provdir-"$i".tar.gz node-deployer/services/"${MACHINE_PROV}"-"$i"/yagna/provdir
done

tar -cf all-logs.tar.gz *.tar.gz


