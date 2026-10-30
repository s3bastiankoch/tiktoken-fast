package bench

import (
	"encoding/json"
	"fmt"
	"math"
	"os"
	"path/filepath"
	"sort"
	"strconv"
	"testing"
	"time"

	fast "github.com/s3bastiankoch/tiktoken-fast/go"
)

// TestFair measures new text the way the Rust, Python and JavaScript
// harnesses of this repository do: the JSON workloads in FAIR_DIR (arrays of
// documents, from bench/data/make_workloads.py) are each encoded once per
// round, and tiktoken-go-fast starts every round with a fresh encoder, so its
// caches hold only what that round has seen. Rounds alternate the arm order;
// a row is the median per-round speed-up over tiktoken-go (the fork point)
// and over tiktoken-go/tokenizer. Every document is checked against
// tiktoken-go first; tokenizer, an independent port, is only counted.
//
// With RAW=<file>, every timed round and every check is appended to that file
// as one JSON line for bench/summarize.py.
//
//	FAIR_DIR=~/tkf-work/main go test -run TestFair -v -timeout 2h .
func TestFair(t *testing.T) {
	dir := os.Getenv("FAIR_DIR")
	if dir == "" {
		t.Skip("set FAIR_DIR to a directory of workload JSON files")
	}
	rounds := 11
	if r, err := strconv.Atoi(os.Getenv("ROUNDS")); err == nil {
		rounds = r
	}
	var raw *os.File
	if p := os.Getenv("RAW"); p != "" {
		f, err := os.OpenFile(p, os.O_CREATE|os.O_APPEND|os.O_WRONLY, 0o644)
		if err != nil {
			t.Fatal(err)
		}
		defer f.Close()
		raw = f
	}
	emit := func(m map[string]any) {
		if raw != nil {
			m["lang"] = "go"
			b, _ := json.Marshal(m)
			raw.Write(append(b, '\n'))
		}
	}
	files, _ := filepath.Glob(filepath.Join(dir, "*.json"))
	sort.Strings(files)
	for _, e := range encodings() {
		for _, f := range files {
			raw, err := os.ReadFile(f)
			if err != nil {
				t.Fatal(err)
			}
			var docs []string
			if err := json.Unmarshal(raw, &docs); err != nil {
				t.Fatal(err)
			}
			n, tokOff := 0, 0
			for i, d := range docs {
				n += len(d)
				a, b := e.fast.EncodeOrdinary(d), e.up.EncodeOrdinary(d)
				c, _, _ := e.tok.Encode(d)
				if !sameUint(c, b) {
					tokOff++ // tokenizer is an independent port; its errors are reported, not fatal
				}
				if !same(a, b) {
					k := 0
					for k < len(a) && k < len(b) && a[k] == b[k] {
						k++
					}
					t.Fatalf("%s %s doc %d: fast %d tokens, tiktoken-go %d, tokenizer %d; first difference at token %d: fast %v, tiktoken-go %v",
						e.name, filepath.Base(f), i, len(a), len(b), len(c), k, a[k:min(k+5, len(a))], b[k:min(k+5, len(b))])
				}
			}
			wname := filepath.Base(f[:len(f)-5])
			emit(map[string]any{"kind": "check", "enc": e.name, "workload": wname, "arm": "tiktoken-fast", "docs": len(docs), "bytes": n, "mism": 0})
			emit(map[string]any{"kind": "check", "enc": e.name, "workload": wname, "arm": "tokenizer", "docs": len(docs), "bytes": n, "mism": tokOff})
			armNames := [3]string{"tiktoken-fast", "tiktoken-go", "tokenizer"}
			var rUp, rTok []float64
			for r := 0; r < rounds; r++ {
				fresh, err := fast.GetEncoding(e.name)
				if err != nil {
					t.Fatal(err)
				}
				arms := []func() time.Duration{
					func() time.Duration { return timeAll(docs, func(s string) { fresh.EncodeOrdinary(s) }) },
					func() time.Duration { return timeAll(docs, func(s string) { e.up.EncodeOrdinary(s) }) },
					func() time.Duration { return timeAll(docs, func(s string) { e.tok.Encode(s) }) },
				}
				var d [3]time.Duration
				for k := 0; k < 3; k++ {
					i := (k + r) % 3
					d[i] = arms[i]()
				}
				for i, x := range d {
					emit(map[string]any{"kind": "time", "enc": e.name, "workload": wname, "scenario": "stream", "call": "encode_ordinary", "arm": armNames[i], "round": r, "seconds": x.Seconds(), "bytes": n})
				}
				rUp = append(rUp, float64(d[1])/float64(d[0]))
				rTok = append(rTok, float64(d[2])/float64(d[0]))
			}
			fmt.Printf("%-11s %-18s stream over tiktoken-go %6.2fx  over tokenizer %6.2fx  (%d B; tokenizer differs on %d of %d documents)\n",
				e.name, wname, median(rUp), median(rTok), n, tokOff, len(docs))
		}
	}
}

func timeAll(docs []string, f func(string)) time.Duration {
	s := time.Now()
	for _, d := range docs {
		f(d)
	}
	return time.Since(s)
}

func same(a, b []int) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if a[i] != b[i] {
			return false
		}
	}
	return true
}

func sameUint(a []uint, b []int) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if int(a[i]) != b[i] {
			return false
		}
	}
	return true
}

func median(v []float64) float64 {
	s := append([]float64(nil), v...)
	sort.Float64s(s)
	if len(s) == 0 {
		return math.NaN()
	}
	return s[len(s)/2]
}
