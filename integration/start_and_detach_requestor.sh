#!/bin/bash

set -x

# Start yagna nodes
./start_requestor.sh &

sleep 10

# Start vanity service
./start_vanity.sh &
