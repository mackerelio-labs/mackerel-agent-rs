#!/bin/bash -eux

if [ ! -d node_modules ] ; then
  rsync -au /tmp/target .
fi
set +u
if [ -z "$1" ] ; then
  set -u
  make start
else
  set -u
  # shellcheck disable=SC2068
  exec "$@"
fi
