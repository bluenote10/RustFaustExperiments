#!/bin/bash

set -e

if [[ "$#" -eq 1 && "$1" == "fastmath" ]]; then
  echo "Benchmarking with fastmath"
  export EXTRA_CPP_ARGS="-ffast-math"
  RESULT_FILE=cpp_fastmath.json
else
  echo "Benchmarking without fastmath"
  export EXTRA_CPP_ARGS=""
  RESULT_FILE=cpp_no_fastmath.json
fi

cd $(dirname $0)

./gen_and_run.sh ../dsp/copy1.dsp "../results/copy1/$RESULT_FILE"
./gen_and_run.sh ../dsp/copy2.dsp "../results/copy2/$RESULT_FILE"
./gen_and_run.sh ../dsp/delay.dsp "../results/delay/$RESULT_FILE"
./gen_and_run.sh ../dsp/math.dsp "../results/math/$RESULT_FILE"
./gen_and_run.sh ../dsp/karplus32.dsp "../results/karplus32/$RESULT_FILE"
./gen_and_run.sh ../dsp/reverb.dsp "../results/reverb/$RESULT_FILE"
./gen_and_run.sh ../dsp/osci.dsp "../results/osci/$RESULT_FILE"
