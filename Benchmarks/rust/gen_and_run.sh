#!/bin/bash

# This script needs to know the root of the Faust installation -- adjust as needed:
FAUSTROOT=${HOME}/bin/faust

set -e

if [ "$#" -ne 2 ]; then
  echo "Usage: $0 <DSP-FILE> <RESULT-FILE>" >&2
  exit 1
fi
DSP_FILE=$(readlink -m "$1")
RESULT_FILE=$(readlink -m "$2")

mkdir -p $(dirname $RESULT_FILE)

cd $(dirname $0)

echo -e "\nBenchmarking: $DSP_FILE"

# Environment setup
PATH=${FAUSTROOT}/bin:${PATH}
. ${FAUSTROOT}/bin/faustpath
echo "FAUSTLDDIR: $FAUSTLDDIR"
echo "FAUSTLIB:   $FAUSTLIB"
echo "FAUSTINC:   $FAUSTINC"
echo "FAUSTARCH:  $FAUSTARCH"

DSP_FILE_BASENAME=$(basename $DSP_FILE)
FILE_WITHOUT_EXTENSION=${DSP_FILE_BASENAME%.dsp}
RUST_FILE=${FILE_WITHOUT_EXTENSION}.rs

faust -a ./architecture/benchmark.rs -lang rust "$DSP_FILE" -o ./src/bin/$RUST_FILE --class-name Dsp
rustfmt --config max_width=120 ./src/bin/$RUST_FILE

cargo run --bin ${FILE_WITHOUT_EXTENSION} --release -- "$RESULT_FILE"
