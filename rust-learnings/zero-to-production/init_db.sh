#!/usr/bin/env bash
set -x # output all commands as echo
set -eo pipefail # exit if any command fails

# VAR="${ENV_VAR:=default}"
DB_USER="${POSTGRES_USER:=postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=6000}"

docker run \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME}
    -p "${DB_PORT}":5432 \
    -d postgres \
    postgres -N 1000 # increase maximum number of connections