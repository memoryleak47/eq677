#!/bin/bash

RUSTFLAGS="-C force-frame-pointers=yes" cargo flamegraph
[ -f perf.data ] && rm perf.data

firefox flamegraph.svg &
