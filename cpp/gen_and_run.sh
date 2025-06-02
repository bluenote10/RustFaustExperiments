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

# Version check
echo "Faust version: $(faust --version)"
echo "g++ version: $(g++ --version)"

DSP_FILE_BASENAME=$(basename $DSP_FILE)
FILE_WITHOUT_EXTENSION=${DSP_FILE_BASENAME%.dsp}
CPP_FILE=${FILE_WITHOUT_EXTENSION}.cpp

set -x

faust -a ./console-bench.cpp "$DSP_FILE" -o ./src/$CPP_FILE --class-name Dsp

# It looks like -march=native -ffast-math are the most relevant optimizations?
g++ -O3 -march=native -mfpmath=sse -msse -msse2 -msse3 -ftree-vectorize $EXTRA_CPP_ARGS \
  -I$FAUSTINC\
  ./src/$CPP_FILE \
  -o ./bin/$FILE_WITHOUT_EXTENSION

./bin/$FILE_WITHOUT_EXTENSION "$RESULT_FILE"
