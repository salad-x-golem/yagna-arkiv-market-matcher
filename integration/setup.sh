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

YAGNA_VERSION="pre-rel-v0.17.6-preview.noarkiv.14"

set -x

# rm -fr venv
# rm -fr node-deployer
# rm -fr req-deployer

# Prepare Python environment
python -m venv venv
venv/bin/python -m pip install eth-account requests dotenv

./setup_provider.sh "${NUMBER_OF_NODES}" "${YAGNA_VERSION}"

./setup_requestor.sh "${YAGNA_VERSION}"

# Router
(cd node-deployer/central-net && ./download_router.sh)

