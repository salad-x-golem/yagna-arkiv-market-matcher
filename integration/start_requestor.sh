#!/bin/bash
set -x

# Start router
(cd req-deployer/services/lower-0/yagna && ./yagna-lower-0 --debug service run)