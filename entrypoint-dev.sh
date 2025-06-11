#!/bin/bash


# 1 Create the mnemonic first
# ./entrypoint-dev.sh -m create
# 2 Remove 'export' file in TOFND_HOME
# 3. Run the script again
# ./entrypoint-dev.sh


set -e

OK=0
ERR=1

EXEC_BIN="target/release/tofnd"
TOFND_HOME="./.tofnd"

check_bin() {
    # check in target/release/tofnd
    if [ ! -f "$EXEC_BIN" ]; then
        echo "tofnd binary not found in $EXEC_BIN"
        cargo build --release
    fi

    if [ ! -d "$TOFND_HOME" ]; then
        echo "tofnd home directory not found in $TOFND_HOME"
        mkdir -p $TOFND_HOME
    fi
}

check_bin

ARGS+=" -d $TOFND_HOME"

echo PASSWORD: $PASSWORD
echo ARGS: $ARGS

# execute tofnd daemon
# exec echo ${PASSWORD} | $EXEC_BIN ${ARGS} "$@"; \
exec $EXEC_BIN ${ARGS} "$@"

