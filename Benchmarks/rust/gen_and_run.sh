#!/bin/bash

# This script needs to know the root of the Faust installation -- adjust as needed:
FAUSTROOT=${HOME}/bin/faust

set -e

if [ "$#" -ne 1 ]; then
  echo "Usage: $0 <DSP-FILE>" >&2
  exit 1
fi
DSPFILE=$(readlink -f $1)

cd $(dirname $0)

# Environment setup
PATH=${FAUSTROOT}/bin:${PATH}
. ${FAUSTROOT}/bin/faustpath
echo "FAUSTLDDIR: $FAUSTLDDIR"
echo "FAUSTLIB:   $FAUSTLIB"
echo "FAUSTINC:   $FAUSTINC"
echo "FAUSTARCH:  $FAUSTARCH"

DSP_FILE_BASENAME=$(basename $DSPFILE)
FILE_WITHOUT_EXTENSION=${DSP_FILE_BASENAME%.dsp}
RUST_FILE=${FILE_WITHOUT_EXTENSION}.rs

faust -a ./architecture/benchmark.rs -lang rust "$DSPFILE" -o ./src/bin/$RUST_FILE --class-name Dsp
rustfmt --config max_width=120 ./src/bin/$RUST_FILE

cargo run --bin ${FILE_WITHOUT_EXTENSION} --release
