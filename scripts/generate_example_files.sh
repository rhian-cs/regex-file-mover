#!/bin/bash

OUTPUT_PATH="tmp/example1"

mkdir -p "$OUTPUT_PATH"
rm -r "$OUTPUT_PATH"
mkdir "$OUTPUT_PATH"

echo "File 1" > "$OUTPUT_PATH/20230601.txt"
echo "File 2" > "$OUTPUT_PATH/20230602.txt"
echo "File 3" > "$OUTPUT_PATH/20230603.txt"
echo "File 4" > "$OUTPUT_PATH/20230701.txt"
echo "File 5" > "$OUTPUT_PATH/20230702.txt"
echo "File 6" > "$OUTPUT_PATH/20240101.txt"
