#!/usr/bin/env bash

mkdir -p renders
PPMNAME=`date +%Y.%m.%d-%H.%M.%S`
RPATH=`pwd`/renders
PPMPATH=$RPATH/$PPMNAME.ppm
echo "Started rendering $PPMNAME"

./target/$1/rust-rt-one-weekend > $PPMPATH
mogrify -format png $RPATH/$PPMNAME.ppm
echo "Done: " $RPATH/$PPMNAME.png
xdg-open $RPATH/$PPMNAME.png
rm $RPATH/$PPMNAME.ppm
