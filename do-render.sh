#!/usr/bin/env bash

mkdir -p renders
PPMNAME=`date +%Y.%m.%d-%H.%M.%S`
RPATH=`pwd`/renders
PPMPATH=$RPATH/$PPMNAME.ppm
echo "Started rendering $PPMNAME"

RENDERER=biased
#RENDERER=unbiased
#RENDERER=bounces-heatmap
SCENE=cornel_is
#SCENE=cornel_volumes
#SCENE=cornel_instances
#./target/$1/rust-rt-one-weekend | imvr -
time ./target/$1/rust-rt-one-weekend \
  --renderer $RENDERER \
  --bounces 16 --width 400 --height 400 --samples 5000 \
  $SCENE > $PPMPATH
#  cornel_volumes > $PPMPATH
#  --bounces 10 --width 384 --height 216 --samples 100 \
#  --bounces 50 --width 384 --height 216 --samples 100 > $PPMPATH
#  next_week_final > $PPMPATH
#  cornel_instances > $PPMPATH

mogrify -format png $RPATH/$PPMNAME.ppm && rm $PPMPATH
echo "Done: " $RPATH/$PPMNAME.png
xdg-open $RPATH/$PPMNAME.png
