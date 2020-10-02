#!/bin/bash -eux

# /usr/local/bin/precopy_appsync
rsync -auv \
      --delete \
      --exclude='.#*' \
      --exclude='target/*' \
      /host_sync/ /app_sync
