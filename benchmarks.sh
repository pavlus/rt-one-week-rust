#!/usr/bin/env bash

mkdir -p benchmarks

RUSTFLAGS="-C target-cpu=native" cargo build --release --bin rust-rt-one-weekend

hyperfine './target/release/rust-rt-one-weekend --width 64 --height 64 --samples 1000 --bounces 4 {scene}' \
  --warmup 2 \
  --runs 10 \
  --parameter-list scene \
    next_week_final,weekend_final,cornel_instances,cornel_volumes,perlin \
  --export-json ./benchmarks.json \
  --export-markdown ./BENCHMARKS.md

