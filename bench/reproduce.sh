#!/usr/bin/env bash
# Runs the published benchmarks for every language on this Linux machine and
# writes results/<date>-<instance>-<commit>/ with raw rounds, a machine record
# and summary.md. Toolchains: bench/setup-debian.sh installs the pinned ones.
#
#   bench/reproduce.sh            # full run, 11 rounds, about 4 hours
#   QUICK=1 bench/reproduce.sh    # 5 rounds on 5 workloads, about 15 minutes
#
# Every arm is built with default compiler flags (no target-cpu=native), as a
# user installing the package would get it, and runs pinned to one core (CORE,
# default 2). Workload files are checked against bench/data/workloads.sha256,
# so a run that timed different text says so.
set -euo pipefail
unset RUSTFLAGS
here=$(cd "$(dirname "$0")" && pwd)
root=$(dirname "$here")
core=${CORE:-2}
work=${WORK:-$HOME/tkf-work}
rounds=${ROUNDS:-11}
mkdir -p "$work"
pin() { taskset -c "$core" "$@"; }

echo "== tools"
python3 -m venv "$work/venv"
py="$work/venv/bin/python"
export PY="$py"
"$py" -m pip install -q --upgrade pip
"$py" -m pip install -q -r "$here/requirements.txt"
[ -d "$work/TokenDagger" ] || git clone -q --recursive https://github.com/M4THYOU/TokenDagger "$work/TokenDagger"
git -C "$work/TokenDagger" checkout -q 1eed815

echo "== build (default flags)"
(cd "$here/compare" && cargo build --release -q --locked)
(cd "$root/python" && "$work/venv/bin/maturin" build --release -q -i "$py" -o "$work/wheels" && "$py" -m pip install -q --force-reinstall --no-deps "$work"/wheels/tiktoken_fast-*.whl)
"$root/js/build.sh" > /dev/null
"$root/node/build.sh" > /dev/null
(cd "$root/js/bench" && npm ci --silent)
(cd "$root/go/bench" && go build ./... && go vet .)

echo "== data"
[ -d "$work/wiki" ] || "$py" "$here/data/fetch_wikipedia.py" "$work/wiki" 60
[ -f "$work/main/wiki-en.json" ] || "$py" "$here/data/make_workloads.py" --wiki "$work/wiki" --tokendagger "$work/TokenDagger" --out "$work/main"
data="$work/main"
(cd "$data" && sha256sum -c --quiet "$here/data/workloads.sha256") || {
  echo "workload files differ from bench/data/workloads.sha256; set ALLOW_NEW_DATA=1 to run anyway" >&2
  [ "${ALLOW_NEW_DATA:-}" = 1 ] || exit 1
}
if [ "${QUICK:-}" = 1 ]; then
  rounds=5
  data="$work/quick"; rm -rf "$data"; mkdir -p "$data"
  cp "$work"/main/{wiki-en,wiki-zh,wiki-ru,chat-512B,td-code-files}.json "$data/"
fi

tag=$(curl -s -m 1 -H Metadata-Flavor:Google http://metadata.google.internal/computeMetadata/v1/instance/machine-type 2>/dev/null | sed 's|.*/||')
out="$here/results/$(date +%F)-${tag:-$(hostname -s)}-$(git -C "$root" rev-parse --short HEAD)${QUICK:+-quick}"
rm -rf "$out"; mkdir -p "$out/raw"
"$py" "$here/fingerprint.py" start "$out" "$data" "$core"
export ROUNDS=$rounds

echo "== rust";   RAW="$out/raw/rust.jsonl" ENCS=cl100k_base,o200k_base pin "$here/compare/target/release/compare" "$data" > "$out/rust.txt"
echo "== python"; RAW="$out/raw/python.jsonl" pin "$py" "$here/python/dropin.py" "$data" > "$out/python.txt"
echo "== js";     (cd "$root/js/bench" && RAW="$out/raw/js.jsonl" MAX_BYTES=1e15 TKF_NATIVE="$root/node/index.js" pin node bench.mjs "$data" > "$out/js.txt")
echo "== go";     (cd "$root/go/bench" && RAW="$out/raw/go.jsonl" FAIR_DIR="$data" pin go test -count=1 -run '^TestFair$' -v -timeout 6h . > "$out/go.txt")

"$py" "$here/fingerprint.py" end "$out"
"$py" "$here/summarize.py" "$out"
echo "results in $out"
