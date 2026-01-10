#!/bin/bash

set -x

./start_matcher.sh &

sleep 2

# Start yagna nodes
./start_requestor.sh &

sleep 10

# Start vanity service
./start_vanity.sh &
