#!/usr/bin/env bash

mkdir -p benchmarks

#RUSTFLAGS="-C target-cpu=native" cargo build --release --bin rust-rt-one-weekend

hyperfine 'target\release\rust-rt-one-weekend.exe --width 128 --height 128 --bounces 4 {scene}' \
  --warmup 3 \
  --runs 20 \
  --parameter-list scene \
"--renderer biased -s 62 next_week_final,\
--renderer unbiased -s 220 weekend_final,\
--renderer biased -s 2950 cornel_instances,\
--renderer biased -s 1900 cornel_volumes,\
--renderer unbiased -s 3300 perlin" \
  --export-json ./benchmarks.json \
  --export-markdown ./BENCHMARKS.md

