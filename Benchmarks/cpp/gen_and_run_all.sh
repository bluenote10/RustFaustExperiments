#!/bin/bash

export EXTRA_CPP_ARGS=""
#export EXTRA_CPP_ARGS="-ffast-math"

./gen_and_run.sh ../dsp/copy1.dsp
./gen_and_run.sh ../dsp/copy2.dsp
./gen_and_run.sh ../dsp/math.dsp
./gen_and_run.sh ../dsp/karplus32.dsp
./gen_and_run.sh ../dsp/reverb.dsp
