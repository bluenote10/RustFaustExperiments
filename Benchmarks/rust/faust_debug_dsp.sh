#!/bin/bash

set -e

if [ "$#" -ne 1 ]; then
  echo "Usage: $0 <DSP-FILE>" >&2
  exit 1
fi

DSPFILE=$(readlink -f $1)

cd $(dirname $0)

PATH=${HOME}/bin/faust/bin:${PATH}

. ${HOME}/bin/faust/bin/faustpath

echo "FAUSTLDDIR: $FAUSTLDDIR"
echo "FAUSTLIB:   $FAUSTLIB"
echo "FAUSTINC:   $FAUSTINC"
echo "FAUSTARCH:  $FAUSTARCH"

faust -a ./architecture/debug_advanced.rs -lang rust "$DSPFILE" -o ./src/bin/debug_faust_dsp.rs --class-name Dsp

cargo run --bin debug_faust_dsp --release

cd ..
./py_dev/plot_debug_log.py
