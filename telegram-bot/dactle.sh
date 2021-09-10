#!/bin/bash
#
#


BIN_PATH=$(dirname $(readlink -f $0))

npm run build

node ${BIN_PATH}/lib/index.js $@
