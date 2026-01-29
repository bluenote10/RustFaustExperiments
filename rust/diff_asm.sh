#!/bin/bash
set -e

if [ $# -ne 2 ]; then
    echo "Usage: $0 <target_folder_1> <target_folder_2>"
    echo "Example: $0 target_20260129_140000 target"
    exit 1
fi

DIR1="$1/release"
DIR2="$2/release"

for asm_file in "$DIR1"/*.asm; do
    name=$(basename "$asm_file")
    if [ ! -f "$DIR2/$name" ]; then
        echo "=== $name: only in $DIR1 ==="
        continue
    fi
    if diff -q "$asm_file" "$DIR2/$name" > /dev/null 2>&1; then
        echo "=== $name: identical ==="
    else
        echo "=== $name: DIFFERS ==="
        diff -u "$asm_file" "$DIR2/$name" || true
    fi
done
