package tiktoken

import (
	"math/rand"
	"testing"
)

// TestMergeIntoMatchesReference checks the allocation-free merge, both the
// scanning and the heap form, against the upstream merge loop on random
// pieces: bytes, ASCII words and CJK runs, up to mergeMax long.
func TestMergeIntoMatchesReference(t *testing.T) {
	enc := testEncoding(t)
	bp := enc.bpe
	c := bp.vocab.pool.Get().(*pieceCache)
	defer bp.vocab.pool.Put(c)
	rng := rand.New(rand.NewSource(42))
	gen := func() []byte {
		n := 1 + rng.Intn(mergeMax)
		switch rng.Intn(3) {
		case 0:
			b := make([]byte, n)
			rng.Read(b)
			return b
		case 1:
			b := make([]byte, 0, n)
			for len(b) < n {
				b = append(b, byte('a'+rng.Intn(26)))
			}
			return b
		default:
			b := make([]byte, 0, n+3)
			for len(b) < n {
				b = append(b, string(rune(0x4E00+rng.Intn(0x2000)))...)
			}
			return b
		}
	}
	for i := 0; i < 3000; i++ {
		piece := gen()
		want := bytePairEncode(piece, bp.encoder)
		got := bp.vocab.mergeInto(c, nil, string(piece))
		if len(got) != len(want) {
			t.Fatalf("piece %q: got %v want %v", piece, got, want)
		}
		for j := range got {
			if got[j] != want[j] {
				t.Fatalf("piece %q: got %v want %v", piece, got, want)
			}
		}
		// twice, so the pair and piece caches are exercised warm
		got = c.merge(bp.vocab, nil, string(piece))
		for j := range got {
			if got[j] != want[j] {
				t.Fatalf("warm piece %q: got %v want %v", piece, got, want)
			}
		}
	}
}
