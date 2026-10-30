package tiktoken

// pieceCache remembers the tokens of pieces that missed the vocabulary and
// went through the merge loop. Text repeats its words, so most merges are
// repeats too. Direct-mapped and keyed by a hash of the piece, with the piece
// kept for an exact comparison. Entries are fixed-size and pointer-free, so
// the cache never allocates and the collector never walks it; pieces longer
// than cachePiece bytes or more than cacheTok tokens bypass it. Each
// vocabulary owns a sync.Pool of caches, so a goroutine works on a cache of
// its own, with no sharing and no locking. The cache also carries the merge
// loop's scratch arrays, so a merge zeroes nothing.
const (
	cacheBits  = 13
	cacheSize  = 1 << cacheBits
	cachePiece = 48
	cacheTok   = 24
)

type cacheEntry struct {
	hash  uint64
	n     uint8
	ntok  uint8
	piece [cachePiece]byte
	tok   [cacheTok]int32
}

type pieceCache struct {
	e      [cacheSize]cacheEntry
	pairs  [pairSize]pairEntry
	starts [mergeMax + 1]int32
	ranks  [mergeMax + 1]int32
	toks   [mergeMax + 1]int32
	next   [mergeMax + 1]int32
	prev   [mergeMax + 1]int32
	heap   [3 * mergeMax]uint64
}

// pairEntry remembers whether two adjacent tokens merge, and into what. The
// merge loop asks this for every neighbour pair it creates, and in running
// text the same pairs come up again and again.
const (
	pairBits = 16
	pairSize = 1 << pairBits
)

type pairEntry struct {
	a, b int32
	r    int32 // merged token, or noRank
}

func pairSlot(a, b int32) uint32 {
	h := (uint64(uint32(a))<<32 | uint64(uint32(b))) * 0x9E3779B97F4A7C15
	return uint32(h>>32) & (pairSize - 1)
}

// pairRank returns the rank of the merge of tokens a and b, whose bytes are
// piece[x:y], or noRank when they do not merge.
func (c *pieceCache) pairRank(v *vocab, a, b int32, piece string, x, y int32) int32 {
	e := &c.pairs[pairSlot(a, b)]
	if e.a == a && e.b == b && e.r != 0 {
		return e.r
	}
	r := int32(noRank)
	if t := v.lookup(piece[x:y]); t != noToken {
		r = int32(t)
	}
	e.a, e.b, e.r = a, b, r
	return r
}

// merge appends the tokens of piece, from the cache when it has them.
func (c *pieceCache) merge(v *vocab, out []int, piece string) []int {
	if len(piece) > cachePiece {
		return v.mergeInto(c, out, piece)
	}
	h := uint64(longHash(piece))<<32 | uint64(len(piece))
	h *= 0x9E3779B97F4A7C15
	h ^= h >> 29
	e := &c.e[h&(cacheSize-1)]
	if e.hash == h && int(e.n) == len(piece) && string(e.piece[:e.n]) == piece {
		for i := 0; i < int(e.ntok); i++ {
			out = append(out, int(e.tok[i]))
		}
		return out
	}
	before := len(out)
	out = v.mergeInto(c, out, piece)
	if ntok := len(out) - before; ntok <= cacheTok {
		e.hash, e.n, e.ntok = h, uint8(len(piece)), uint8(ntok)
		copy(e.piece[:], piece)
		for i := 0; i < ntok; i++ {
			e.tok[i] = int32(out[before+i])
		}
	}
	return out
}
