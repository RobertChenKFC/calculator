#!/usr/bin/env bash
# run-checked.sh — run a command, keep stdout/stderr live on the terminal,
# and fail if the command wrote anything to stderr (even if it exited 0).
#
# Usage: ./run-checked.sh your_command --with --args
#
# Written with the help of Claude.

set -eu

if [ "$#" -eq 0 ]; then
    echo "Usage: $0 <command> [args...]" >&2
    exit 2
fi

exec 3>&1
stderr=$( { "$@" 2>&1 1>&3 3>&-; } | tee /dev/stderr )
status=$?
exec 3>&-

if [ "$status" -ne 0 ]; then
    exit "$status"
fi

if [ -n "$stderr" ]; then
    echo "Error: command wrote to stderr" >&2
    exit 1
fi