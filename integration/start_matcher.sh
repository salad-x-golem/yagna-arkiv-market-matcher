#!/bin/bash
set -x

# Start matcher
(cd ../ && ./yagna-offer-server > matcher.log 2>&1 &)