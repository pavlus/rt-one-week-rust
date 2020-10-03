#!/usr/bin/env bash

perf record --call-graph=lbr -- ./target/$1/rust-rt-one-weekend  \
  --bounces 12 --width 400 --height 400 --samples 40 \
  next_week_final > /dev/null
#  cornel_is > /dev/null
#  cornel_volumes > /dev/null
#  cornel_instances > /dev/null
#  perlin > /dev/null
#  weekend_final > /dev/null
#perf report

