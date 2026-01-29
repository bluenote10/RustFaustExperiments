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
echo "Rust version: $(rustc --version)"

DSP_FILE_BASENAME=$(basename $DSP_FILE)
FILE_WITHOUT_EXTENSION=${DSP_FILE_BASENAME%.dsp}
RUST_FILE=${FILE_WITHOUT_EXTENSION}.rs

set -x

faust -a ./architecture/benchmark.rs -lang rust "$DSP_FILE" -o ./src/bin/$RUST_FILE --class-name Dsp
rustfmt --config max_width=120 ./src/bin/$RUST_FILE

# Using `-v` is helpful to see full rustc calls with all flags injected by cargo.
cargo run -v --bin ${FILE_WITHOUT_EXTENSION} --release -- "$RESULT_FILE"

# Compute binary sha256 and size for tracking codegen changes
BINARY_PATH="./target/release/${FILE_WITHOUT_EXTENSION}"
BINARY_SHA256=$(sha256sum "$BINARY_PATH" | awk '{print $1}')
BINARY_SIZE=$(stat --format=%s "$BINARY_PATH")
BINARY_INFO_FILE="${RESULT_FILE%.json}_binary_info.json"
echo "{\"sha256\": \"${BINARY_SHA256}\", \"size\": ${BINARY_SIZE}}" > "$BINARY_INFO_FILE"
echo "Binary: sha256=${BINARY_SHA256} size=${BINARY_SIZE}"

# Dump disassembly for diffing against previous versions
objdump -d "$BINARY_PATH" > "${BINARY_PATH}.asm"
echo "Disassembly written to ${BINARY_PATH}.asm"
