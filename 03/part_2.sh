#!/bin/bash

grep -Po 'mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don.t\(\)' ./input.txt | gawk '
BEGIN {multiplying=1}
/mul/ {if(multiplying){match($0, /mul\(([0-9]+),([0-9]+)\)/, operands); sum += operands[1] * operands[2]}}
/do\(\)/ {multiplying=1}
/don.t/ {multiplying=0}
END {print(sum)}'
