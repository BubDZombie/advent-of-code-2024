#!/bin/bash
awk '{left[$1] = $1; right[$2]++} END {similartiy=0; for(i in left){similarity += left[i] * right[i]}; print(similarity)}' input.txt
