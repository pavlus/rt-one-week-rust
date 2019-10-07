#!/usr/bin/env bash

perf record --call-graph=lbr ./target/$1/rust-rt-one-weekend > /dev/null
perf report
