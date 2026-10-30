package tiktoken

import "sync"

// vocab is the token table the hot path reads. Go's map hashes every string
// key through the runtime; pieces are short, and most are at most eight bytes,
// so those are looked up by their packed bytes in an integer table instead.
// Longer keys live in a second open-addressing table over one byte arena.
// Both tables keep one 16-byte entry per slot, so a probe touches one line.
type vocab struct {
	short  []shortEntry // pieces of 1..8 bytes
	smask  uint64
	sshift uint        // 64 - log2(len(short))
	long   []longEntry // pieces of 9+ bytes
	lmask  uint64
	arena  []byte
	pool   sync.Pool // of *pieceCache
	// byteTok is the token of each single byte; every shipped vocabulary has all 256
	byteTok [256]int32
}

type shortEntry struct {
	key uint64 // the bytes, little-endian
	val int32  // token, or noToken for an empty slot
	n   uint8  // length
}

type longEntry struct {
	hash uint32
	off  uint32
	n    uint32
	val  int32
}

const noToken = -1

func newVocab(encoder map[string]int) *vocab {
	var nShort, nLong, arena int
	for k := range encoder {
		if len(k) <= 8 {
			nShort++
		} else {
			nLong++
			arena += len(k)
		}
	}
	v := &vocab{}
	v.pool.New = func() any { return new(pieceCache) }
	for b := 0; b < 256; b++ {
		v.byteTok[b] = noToken
		if t, ok := encoder[string([]byte{byte(b)})]; ok {
			v.byteTok[b] = int32(t)
		}
	}
	ss := tableSize(nShort * 2 / 3)
	v.short = make([]shortEntry, ss)
	v.smask = uint64(ss - 1)
	for b := 1; 1<<b < ss; b++ {
		v.sshift = uint(64 - b - 1)
	}
	for i := range v.short {
		v.short[i].val = noToken
	}
	ls := tableSize(nLong)
	v.long = make([]longEntry, ls)
	v.lmask = uint64(ls - 1)
	for i := range v.long {
		v.long[i].val = noToken
	}
	v.arena = make([]byte, 0, arena)
	for k, tok := range encoder {
		if len(k) <= 8 {
			key := pack8(k)
			i := shortHash(key, len(k)) >> v.sshift
			for v.short[i].val != noToken {
				i = (i + 1) & v.smask
			}
			v.short[i] = shortEntry{key: key, val: int32(tok), n: uint8(len(k))}
		} else {
			h := longHash(k)
			i := uint64(h) & v.lmask
			for v.long[i].val != noToken {
				i = (i + 1) & v.lmask
			}
			v.long[i] = longEntry{hash: h, off: uint32(len(v.arena)), n: uint32(len(k)), val: int32(tok)}
			v.arena = append(v.arena, k...)
		}
	}
	return v
}

// tableSize returns a power of two holding n entries at a load of at most one
// half; the short table is sized for three quarters so that it fits L2.
func tableSize(n int) int {
	s := 16
	for s < 2*n {
		s <<= 1
	}
	return s
}

func pack8(s string) uint64 {
	switch len(s) {
	case 8:
		return uint64(s[0]) | uint64(s[1])<<8 | uint64(s[2])<<16 | uint64(s[3])<<24 |
			uint64(s[4])<<32 | uint64(s[5])<<40 | uint64(s[6])<<48 | uint64(s[7])<<56
	case 4:
		return uint64(s[0]) | uint64(s[1])<<8 | uint64(s[2])<<16 | uint64(s[3])<<24
	case 3:
		return uint64(s[0]) | uint64(s[1])<<8 | uint64(s[2])<<16
	case 2:
		return uint64(s[0]) | uint64(s[1])<<8
	case 1:
		return uint64(s[0])
	}
	var k uint64
	for i := 0; i < len(s); i++ {
		k |= uint64(s[i]) << (8 * uint(i))
	}
	return k
}

// shortHash is one multiply; the table index is taken from the top bits,
// which a product mixes well (Fibonacci hashing).
func shortHash(key uint64, n int) uint64 {
	return (key ^ uint64(n)<<56) * 0x9E3779B97F4A7C15
}

func longHash(s string) uint32 {
	var h uint64 = 0x9E3779B97F4A7C15 ^ uint64(len(s))
	for len(s) >= 8 {
		w := uint64(s[0]) | uint64(s[1])<<8 | uint64(s[2])<<16 | uint64(s[3])<<24 |
			uint64(s[4])<<32 | uint64(s[5])<<40 | uint64(s[6])<<48 | uint64(s[7])<<56
		h = (h ^ w) * 0xBF58476D1CE4E5B9
		h ^= h >> 31
		s = s[8:]
	}
	if len(s) > 0 {
		h = (h ^ pack8(s)) * 0x94D049BB133111EB
		h ^= h >> 29
	}
	return uint32(h ^ (h >> 32))
}

// lookup returns the token of piece s, or noToken.
func (v *vocab) lookup(s string) int {
	if len(s) <= 8 {
		if len(s) == 0 {
			return noToken
		}
		key := pack8(s)
		i := shortHash(key, len(s)) >> v.sshift
		for {
			e := &v.short[i]
			if e.key == key && int(e.n) == len(s) {
				return int(e.val)
			}
			if e.val == noToken {
				return noToken
			}
			i = (i + 1) & v.smask
		}
	}
	h := longHash(s)
	i := uint64(h) & v.lmask
	for {
		e := &v.long[i]
		if e.val == noToken {
			return noToken
		}
		if e.hash == h && int(e.n) == len(s) && string(v.arena[e.off:e.off+e.n]) == s {
			return int(e.val)
		}
		i = (i + 1) & v.lmask
	}
}
