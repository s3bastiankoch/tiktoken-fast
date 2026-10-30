package tiktoken

import "math"

// bytePairMerge is the upstream merge loop, kept for the generic callers and
// the equivalence tests. The hot path uses mergeInto.
func bytePairMerge[T any](piece []byte, ranks map[string]int, f func(start, end int) T) []T {
	parts := make([][2]int, len(piece)+1)
	for i := 0; i < len(parts); i++ {
		parts[i][0], parts[i][1] = i, math.MaxInt // use max int as sentinel
	}

	getRank := func(startIdx, skip int) int {
		if startIdx+skip+2 < len(parts) {
			b := piece[parts[startIdx][0]:parts[startIdx+skip+2][0]]
			rank, ok := ranks[string(b)]
			if ok {
				return rank
			}
		}
		return -1 // use -1 to represent None
	}

	for i := 0; i < len(parts)-2; i++ {
		if rank := getRank(i, 0); rank >= 0 {
			parts[i][1] = rank
		}
	}

	for len(parts) > 1 {
		minRank, minIdx := math.MaxInt, -1
		for i := 0; i < len(parts)-1; i++ {
			if parts[i][1] < minRank {
				minRank, minIdx = parts[i][1], i
			}
		}

		if minRank < math.MaxInt {
			i := minIdx
			rank := getRank(i, 1)
			if rank >= 0 {
				parts[i][1] = rank
			} else {
				parts[i][1] = math.MaxInt
			}
			if i > 0 {
				rk := getRank(i-1, 1)
				if rk >= 0 {
					parts[i-1][1] = rk
				} else {
					parts[i-1][1] = math.MaxInt
				}
			}
			parts = append(parts[:i+1], parts[i+2:]...)
		} else {
			break
		}
	}

	out := make([]T, len(parts)-1)
	for i := 0; i < len(out); i++ {
		out[i] = f(parts[i][0], parts[i+1][0])
	}
	return out
}

func bytePairEncode(piece []byte, ranks map[string]int) []int {
	if len(piece) == 1 {
		v := ranks[string(piece)]
		return []int{v}
	}
	return bytePairMerge(piece, ranks, func(start, end int) int {
		return ranks[string(piece[start:end])]
	})
}

// mergeMax is the piece length the merge handles in the cache's scratch arrays.
const mergeMax = 256

const noRank = math.MaxInt32

// mergeInto appends the byte-pair encoding of piece to out. Same merges in the
// same order as bytePairMerge: the lowest-ranked adjacent pair goes first, ties
// to the leftmost. Boundaries, ranks and part tokens live in arrays the cache
// lends, so a piece costs no allocation; a rank is the token the pair merges
// into, so the cache's pair table answers most rank questions without a
// lookup in the vocabulary.
func (v *vocab) mergeInto(c *pieceCache, out []int, piece string) []int {
	n := len(piece)
	if n > heapFrom && n <= mergeMax {
		return v.mergeHeap(c, out, piece)
	}
	var starts, ranks, toks []int32
	if n <= mergeMax {
		starts, ranks, toks = c.starts[:n+1], c.ranks[:n+1], c.toks[:n+1]
	} else {
		starts, ranks, toks = make([]int32, n+1), make([]int32, n+1), make([]int32, n+1)
	}
	for i := 0; i < n; i++ {
		starts[i] = int32(i)
		toks[i] = v.byteTok[piece[i]]
	}
	starts[n] = int32(n)
	ranks[n], ranks[n-1] = noRank, noRank
	for i := 0; i+2 <= n; i++ {
		ranks[i] = c.pairRank(v, toks[i], toks[i+1], piece, starts[i], starts[i+2])
	}
	m := n + 1 // number of boundaries, parts = m-1
	for m > 2 {
		minRank, minIdx := int32(noRank), -1
		for i := 0; i < m-1; i++ {
			if ranks[i] < minRank {
				minRank, minIdx = ranks[i], i
			}
		}
		if minIdx < 0 {
			break
		}
		i := minIdx
		// merge parts i and i+1: drop boundary i+1, the merged part's token is its rank
		copy(starts[i+1:m-1], starts[i+2:m])
		copy(ranks[i+1:m-1], ranks[i+2:m])
		copy(toks[i+1:m-1], toks[i+2:m])
		m--
		toks[i] = minRank
		if i+2 < m {
			ranks[i] = c.pairRank(v, toks[i], toks[i+1], piece, starts[i], starts[i+2])
		} else {
			ranks[i] = noRank
		}
		if i > 0 {
			ranks[i-1] = c.pairRank(v, toks[i-1], toks[i], piece, starts[i-1], starts[i+1])
		}
	}
	for i := 0; i < m-1; i++ {
		out = append(out, int(toks[i]))
	}
	return out
}

// heapFrom is the piece length from which the merge keeps its candidate pairs
// in a heap instead of scanning them. The scan is linear per merge, so a long
// piece costs its length squared; the heap makes it n log n.
const heapFrom = 40

// mergeHeap is mergeInto for long pieces. Parts form a linked list; the heap
// holds (rank, start) of every pair that was ever a candidate, lowest rank and
// then leftmost first, which is the order the scan would pick. An entry whose
// rank no longer matches its part is stale and skipped.
func (v *vocab) mergeHeap(c *pieceCache, out []int, piece string) []int {
	n := len(piece)
	starts, ranks, toks := c.starts[:n], c.ranks[:n], c.toks[:n]
	next, prev := c.next[:n], c.prev[:n]
	h := c.heap[:0]
	for i := 0; i < n; i++ {
		starts[i] = int32(i)
		toks[i] = v.byteTok[piece[i]]
		next[i], prev[i] = int32(i+1), int32(i-1)
		ranks[i] = noRank
	}
	next[n-1] = -1
	for i := 0; i+1 < n; i++ {
		if r := c.pairRank(v, toks[i], toks[i+1], piece, int32(i), int32(i+2)); r != noRank {
			ranks[i] = r
			h = heapPush(h, uint64(uint32(r))<<32|uint64(i))
		}
	}
	// end of part i is the start of next[i], or n
	end := func(i int32) int32 {
		if j := next[i]; j >= 0 {
			return starts[j]
		}
		return int32(n)
	}
	for len(h) > 0 {
		var e uint64
		h, e = heapPop(h)
		r, i := int32(e>>32), int32(uint32(e))
		if ranks[i] != r || next[i] < 0 {
			continue // stale, or part i no longer has a right neighbour
		}
		j := next[i]
		// merge i and j: i absorbs j
		next[i] = next[j]
		if next[j] >= 0 {
			prev[next[j]] = i
		}
		toks[i] = r
		ranks[j] = noRank
		if k := next[i]; k >= 0 {
			ranks[i] = c.pairRank(v, toks[i], toks[k], piece, starts[i], end(k))
			if ranks[i] != noRank {
				h = heapPush(h, uint64(uint32(ranks[i]))<<32|uint64(i))
			}
		} else {
			ranks[i] = noRank
		}
		if p := prev[i]; p >= 0 {
			ranks[p] = c.pairRank(v, toks[p], toks[i], piece, starts[p], end(i))
			if ranks[p] != noRank {
				h = heapPush(h, uint64(uint32(ranks[p]))<<32|uint64(p))
			}
		}
	}
	for i := int32(0); i >= 0; i = next[i] {
		out = append(out, int(toks[i]))
	}
	return out
}

func heapPush(h []uint64, x uint64) []uint64 {
	h = append(h, x)
	i := len(h) - 1
	for i > 0 {
		p := (i - 1) / 2
		if h[p] <= h[i] {
			break
		}
		h[p], h[i] = h[i], h[p]
		i = p
	}
	return h
}

func heapPop(h []uint64) ([]uint64, uint64) {
	top := h[0]
	last := len(h) - 1
	h[0] = h[last]
	h = h[:last]
	i := 0
	for {
		l, r := 2*i+1, 2*i+2
		m := i
		if l < len(h) && h[l] < h[m] {
			m = l
		}
		if r < len(h) && h[r] < h[m] {
			m = r
		}
		if m == i {
			break
		}
		h[m], h[i] = h[i], h[m]
		i = m
	}
	return h, top
}
