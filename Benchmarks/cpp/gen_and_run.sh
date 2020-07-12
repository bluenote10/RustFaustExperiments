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
CPP_FILE=${FILE_WITHOUT_EXTENSION}.cpp

set -x

faust -a ./console-bench.cpp "$DSPFILE" -o ./src/$CPP_FILE --class-name Dsp

g++ -O3 -O3 -march=native -mfpmath=sse -msse -msse2 -msse3 -ftree-vectorize $EXTRA_CPP_ARGS \
  -I$FAUSTINC\
  ./src/$CPP_FILE \
  -o ./bin/$FILE_WITHOUT_EXTENSION

# It looks like -march=native -ffast-math are the most relevant optimizations?

./bin/$FILE_WITHOUT_EXTENSION
