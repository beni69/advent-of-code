#!/bin/sh
echo part one
awk '{if ($1>prev){sol++};prev=$1}END{print sol-1}' $1
echo part two
awk '{if(NR>3&&$1>p3){sol++};p3=p2;p2=p1;p1=$1}END{print sol-1}' $1
