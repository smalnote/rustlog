#!/bin/sh

set -e

if [ "$#" -eq 0 ]; then
    exec /app/server
else
    exec "$@"
fi
