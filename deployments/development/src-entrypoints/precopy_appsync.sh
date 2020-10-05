#!/bin/bash

set -ex

# /usr/local/bin/precopy_appsync
rsync -auv \
      --delete \
      --exclude='.#*' \
      --exclude='target/*' \
      /host_sync/ /app_sync

chmod +x /app_sync/deployments/development/entrypoint.sh
