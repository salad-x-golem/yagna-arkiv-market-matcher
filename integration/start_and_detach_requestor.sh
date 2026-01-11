#!/bin/bash

set -x

# Start yagna nodes
./start_requestor.sh &

sleep 15

# Start vanity service
./start_vanity.sh &
