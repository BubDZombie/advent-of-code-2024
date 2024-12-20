#!/bin/bash
grep -Po 'mul\([0-9]{1,3},[0-9]{1,3}\)' ./input.txt | sed -e 's/mul(//' -e 's/)//' | awk -F ',' '{sum += $1 * $2} END {print(sum)}'
