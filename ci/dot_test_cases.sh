#!/usr/bin/env bash

set -e

for x in $(find test_cases -name "*.flatdata")
do
echo $x
./generator -s $x -g dot -O test.dot
dot -Tsvg -O test.dot
echo Done
done