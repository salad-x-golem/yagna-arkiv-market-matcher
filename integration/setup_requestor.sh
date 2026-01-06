#!/bin/bash

YAGNA_VERSION=${YAGNA_VERSION}

MACHINE_REQ=${MACHINE_REQ:-lower}
MACHINE_REQ_SECRET=${MACHINE_REQ_SECRET:-bca321}
CENTRAL_NET_HOSTS=${CENTRAL_NET_HOSTS:-"127.0.0.1:6976"}

git clone git@github.com:salad-x-golem/req-deployer.git
(cd req-deployer && git clean -fdx && git reset --hard && git checkout main && git pull)

(cd req-deployer && ../venv/bin/python keys.py 1)
(cd req-deployer && mkdir "${MACHINE_REQ}_keys" && mv generated_keys/keys.txt "${MACHINE_REQ}_keys/${MACHINE_REQ}.keys" )
(cd req-deployer && printf "NODE_PREFIX=%s\nNODE_SECRET=%s\nNO_SERVICES=true\nYAGNA_VERSION=${YAGNA_VERSION}\nCENTRAL_NET_HOSTS=${CENTRAL_NET_HOSTS}" "${MACHINE_REQ}" "${MACHINE_REQ_SECRET}" > .env )
(cd req-deployer && ../venv/bin/python bootstrap.py)

(cd req-deployer && ./setup-all.sh)