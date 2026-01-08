#!/bin/bash
set -x

# Start router
(cd ../ && cargo run -p yagna-offer-server > matcher.log 2>&1 &)