#!/usr/bin/env bash

mkdir -p renders
PPMNAME=`date +%Y.%m.%d-%H.%M.%S`
RPATH=`pwd`/renders
PPMPATH=$RPATH/$PPMNAME.ppm
echo "Started rendering $PPMNAME"

#./target/$1/rust-rt-one-weekend | imvr -
time ./target/$1/rust-rt-one-weekend \
  --bounces 16 --width 1000 --height 1000 --samples 2000 \
  cornel_is > $PPMPATH
#  cornel_volumes > $PPMPATH
#  --bounces 10 --width 384 --height 216 --samples 100 \
#  --bounces 50 --width 384 --height 216 --samples 100 > $PPMPATH
#  next_week_final > $PPMPATH
#  cornel_instances > $PPMPATH

mogrify -format png $RPATH/$PPMNAME.ppm && rm $PPMPATH
echo "Done: " $RPATH/$PPMNAME.png
xdg-open $RPATH/$PPMNAME.png
