# Results

One folder per run, named `<date>-<machine>-<commit>`. The current figures in
the README come from the three `c3`, `c3d` and `c4a` folders; the others are
earlier runs with older harnesses, kept for reference.

The public history starts with a single commit; the commits these runs name
(`a70b9ff` for Intel, `3f3699c` for AMD and ARM, and earlier ids in the older
folders) are from development before it and are not published. Against that
first public commit, the library (`src/`, `python/`, `js/src`, `node/src`,
`go/`) and the benchmark harnesses (`bench/compare`, `bench/python`,
`js/bench/bench.mjs`, `go/bench/fair_test.go`, `bench/reproduce.sh`) are
unchanged from `3f3699c`. What changed: the input generator (it writes the
same files, checked by `data/workloads.sha256`), the summary formatting, and
the `fastarm` benchmark server, which moved to `examples/`.
