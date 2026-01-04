#!/bin/bash
set -x

(cd node-deployer/services/upper-0/yagna && ./yagna-upper-0 service run)&
(cd node-deployer/services/upper-1/yagna && ./yagna-upper-1 service run)&
(cd node-deployer/services/upper-2/yagna && ./yagna-upper-2 service run)&
(cd node-deployer/services/upper-3/yagna && ./yagna-upper-3 service run)&
(cd node-deployer/services/upper-4/yagna && ./yagna-upper-4 service run)&
(cd node-deployer/services/upper-5/yagna && ./yagna-upper-5 service run)&
(cd node-deployer/services/upper-6/yagna && ./yagna-upper-6 service run)&
(cd node-deployer/services/upper-7/yagna && ./yagna-upper-7 service run)&
(cd node-deployer/services/upper-8/yagna && ./yagna-upper-8 service run)&
(cd node-deployer/services/upper-9/yagna && ./yagna-upper-9 service run)&
