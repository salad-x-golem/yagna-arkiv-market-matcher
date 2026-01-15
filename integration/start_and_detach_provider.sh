#!/bin/bash
if [ -z "$1" ]; then
  echo "usage: $0 N"
  exit 1
fi

NUMBER_OF_NODES=$1
if [ "$NUMBER_OF_NODES" -lt 0 ]; then
  echo "no nodes to setup"
  exit 0
fi

set -x

# Start yagna nodes
./start_provider.sh "${NUMBER_OF_NODES}" &
