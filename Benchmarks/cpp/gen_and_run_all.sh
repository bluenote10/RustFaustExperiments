#!/bin/bash

set -e

export EXTRA_CPP_ARGS=""
# export EXTRA_CPP_ARGS="-ffast-math"

cd $(dirname $0)

RESULT_FILE=cpp_no_fastmath.json
# RESULT_FILE=cpp_fastmath.json

./gen_and_run.sh ../dsp/copy1.dsp "../results/copy1/$RESULT_FILE"
./gen_and_run.sh ../dsp/copy2.dsp "../results/copy2/$RESULT_FILE"
./gen_and_run.sh ../dsp/delay.dsp "../results/delay/$RESULT_FILE"
./gen_and_run.sh ../dsp/math.dsp "../results/math/$RESULT_FILE"
./gen_and_run.sh ../dsp/karplus32.dsp "../results/karplus32/$RESULT_FILE"
./gen_and_run.sh ../dsp/reverb.dsp "../results/reverb/$RESULT_FILE"
