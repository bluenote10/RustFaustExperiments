#!/bin/bash

set -e

cd $(dirname $0)

./rust/gen_and_run_all.sh
./cpp/gen_and_run_all.sh
./cpp/gen_and_run_all.sh fastmath
./gen_summary_table.py
