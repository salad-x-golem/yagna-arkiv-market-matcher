#!/bin/bash
set -x

# Start router
(cp ya-sb-router router-upper)
(./router-upper -l tcp://0.0.0.0:6976)