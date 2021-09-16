#!/bin/bash
#
#


BIN_PATH=$(dirname $(readlink -f $0))

DIST_PATH=${BIN_PATH}/lib

if [[ ! -d ${DIST_PATH} ]]; then
  npm run build
fi

node ${DIST_PATH}/index.js $@
