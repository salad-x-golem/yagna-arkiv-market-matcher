#!/bin/bash

set -x

MACHINE_PROV=${MACHINE_PROV:-"geode"}

tar -cf requestor.tar.gz req-deployer/services/lower-0/yagna/.env req-deployer/services/lower-0/yagna/yagnadir

tar -cf vanity-log.tar.gz vanity.log
mv ../matcher.log .
tar -cf matcher-log.tar.gz matcher.log

tar -cf all-logs.tar.gz *.tar.gz

