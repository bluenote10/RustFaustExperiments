#!/bin/bash

set -e

RESULT_FILE=rust.json

./gen_and_run.sh ../dsp/copy1.dsp "../results/copy1/$RESULT_FILE"
./gen_and_run.sh ../dsp/copy2.dsp "../results/copy2/$RESULT_FILE"
./gen_and_run.sh ../dsp/delay.dsp "../results/delay/$RESULT_FILE"
./gen_and_run.sh ../dsp/math.dsp "../results/math/$RESULT_FILE"
./gen_and_run.sh ../dsp/karplus32.dsp "../results/karplus32/$RESULT_FILE"
./gen_and_run.sh ../dsp/reverb.dsp "../results/reverb/$RESULT_FILE"
