#!/bin/bash
set -x

# Start router
(cd req-deployer/services/lower-0/vanity && ./start_bun.sh > vanity.log 2>&1)