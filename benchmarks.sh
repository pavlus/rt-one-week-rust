#!/usr/bin/env bash

mkdir -p benchmarks

RUSTFLAGS="-C target-cpu=native" cargo build --release --bin rust-rt-one-weekend

hyperfine './target/release/rust-rt-one-weekend --width 128 --height 128 --bounces 4 {scene}' \
  --warmup 3 \
  --runs 20 \
  --parameter-list scene \
    "-s 62 next_week_final,-s 220 weekend_final,-s 2950 cornel_instances,-s 1900 cornel_volumes,-s 3300 perlin" \
  --export-json ./benchmarks.json \
  --export-markdown ./BENCHMARKS.md

