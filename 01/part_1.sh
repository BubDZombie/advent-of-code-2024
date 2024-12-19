#!/bin/bash
distance=0
paste <(sort input.txt | awk '{print($1)}') <(sort -gk2 input.txt | awk '{print($2)}') |\
while read a b
  do
    d=$(($a-$b))
    d=${d#-}
    let 'distance=distance + d'
    echo "$d $distance"
  done
