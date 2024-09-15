#!/bin/sh

set -e

echo "Entrypoint final ccommand(all args): $@"
exec "$@"
