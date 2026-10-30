#!/usr/bin/env bash
# Runs every benchmark of this directory on one Linux machine and writes the
# results to results/<date>-<commit>/. Expects Rust, Python 3.11+ with venv,
# a C++ compiler and the PCRE2 and Python headers (Debian: build-essential
# python3-dev python3-venv libpcre2-dev). One core (CORE, default 3) is used
# for the single-threaded runs; the thread test uses up to THREADS cores.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
core=${CORE:-3}
threads=${THREADS:-1,2,4}
work=${WORK:-$HOME/tkf-work}
mkdir -p "$work"
out="$here/results/$(date +%F)-$(git -C "$here" rev-parse --short HEAD)"
mkdir -p "$out"
export RUSTFLAGS="-C target-cpu=native"

# tools
python3 -m venv "$work/venv"
py="$work/venv/bin/python"
"$py" -m pip install -q --upgrade pip
"$py" -m pip install -q tiktoken==0.14.0 maturin pyarrow fsspec aiohttp requests
[ -d "$work/TokenDagger" ] || git clone -q --recursive https://github.com/M4THYOU/TokenDagger "$work/TokenDagger"
git -C "$work/TokenDagger" checkout -q 1eed815
"$py" -m pip install -q "$work/TokenDagger"
(cd "$here/python/binding" && "$work/venv/bin/maturin" build --release -q -i "$py" && "$py" -m pip install -q --force-reinstall target/wheels/*.whl)
(cd "$here/compare" && cargo build --release -q)
(cd "$here/threads" && cargo build --release -q)

# data
[ -d "$work/wiki" ] || "$py" "$here/data/fetch_wikipedia.py" "$work/wiki" 60
"$py" "$here/data/make_workloads.py" --wiki "$work/wiki" --tokendagger "$work/TokenDagger" --out "$work/main" --edge-out "$work/edge"
mkdir -p "$work/old"
cp "$work"/main/{wiki-en,wiki-de,wiki-ru,wiki-zh,chat-512B,book-mobydick,book-gosrc,book-hongloumeng}.json "$work/edge"/*.json "$work/old/"

# runs
{ lscpu | grep -E 'Model name|^CPU\(s\)'; uname -r; rustc --version; git -C "$here" rev-parse HEAD; } > "$out/machine.txt"
cmp="$here/compare/target/release/compare"
taskset -c "$core" "$cmp" "$work/main" > "$out/rust-main.txt"
ENCS=cl100k_base,o200k_base taskset -c "$core" "$cmp" "$work/edge" > "$out/rust-edge.txt"
ENCS=p50k_base,r50k_base taskset -c "$core" "$cmp" "$work/old" > "$out/rust-p50k-r50k.txt"
for e in cl100k o200k; do "$here/threads/target/release/threads" "$work/main/chat-512B.json" $e 3 "$threads"; done > "$out/threads.txt"
taskset -c "$core" "$py" "$here/python/bench.py" "$work/main" > "$out/python-main.txt"
echo "results in $out"
