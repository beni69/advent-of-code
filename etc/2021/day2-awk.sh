#!/bin/sh
echo part one
awk 'BEGIN{x=0;y=0} {switch($1){case"forward": x+=$2;break;case"down":y+=$2;break;case"up":y-=$2;break}} END{print x*y}' $1
echo part two
awk 'BEGIN{x=0;y=0;aim=0} {switch($1){case"forward": x+=$2;y+=$2*aim;break;case"down":aim+=$2;break;case"up":aim-=$2;break}} END{print x*y}' $1
