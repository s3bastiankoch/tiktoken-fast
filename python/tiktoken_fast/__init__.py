"""A faster drop-in for tiktoken.

    import tiktoken_fast as tiktoken
    enc = tiktoken.get_encoding("o200k_base")

returns the same token ids as tiktoken. Encodings are tiktoken's own (files,
model names, plugins); only the core that turns text into tokens is replaced.
``patch()`` makes ``tiktoken.get_encoding`` and ``tiktoken.encoding_for_model``
return these encodings for code that imports tiktoken directly.
"""
from __future__ import annotations

import threading
from typing import AbstractSet, Collection, Literal

import tiktoken
import tiktoken.model
import tiktoken.registry
from tiktoken.core import _special_token_regex, raise_disallowed_special_token

from ._core import CoreBPE as _RustCore

__version__ = "0.1.0"
__all__ = [
    "Encoding",
    "get_encoding",
    "encoding_for_model",
    "encoding_name_for_model",
    "list_encoding_names",
    "patch",
    "unpatch",
]

encoding_name_for_model = tiktoken.encoding_name_for_model
list_encoding_names = tiktoken.list_encoding_names


class _Core(_RustCore):
    """The Rust core. Methods it lacks (encode_with_unstable, single-token
    helpers) go to tiktoken's core, built on first use."""

    def __init__(self, mergeable_ranks, special_tokens, pat_str):
        self._args = (mergeable_ranks, special_tokens, pat_str)
        self._fallback = None

    def __getattr__(self, name):
        d = self.__dict__
        if "_args" not in d:
            raise AttributeError(name)
        if d["_fallback"] is None:
            from tiktoken import _tiktoken

            d["_fallback"] = _tiktoken.CoreBPE(*d["_args"])
        return getattr(d["_fallback"], name)


class Encoding(tiktoken.Encoding):
    """tiktoken.Encoding with the Rust core of tiktoken-fast. A pattern the
    core has no scanner for keeps tiktoken's core."""

    def __init__(
        self,
        name: str,
        *,
        pat_str: str,
        mergeable_ranks: dict[bytes, int],
        special_tokens: dict[str, int],
        explicit_n_vocab: int | None = None,
    ):
        self.name = name
        self._pat_str = pat_str
        self._mergeable_ranks = mergeable_ranks
        self._special_tokens = special_tokens
        self.max_token_value = max(max(mergeable_ranks.values()), max(special_tokens.values(), default=0))
        if explicit_n_vocab:
            assert len(mergeable_ranks) + len(special_tokens) == explicit_n_vocab
            assert self.max_token_value == explicit_n_vocab - 1
        self._special_token_values = set(special_tokens.values())
        try:
            self._core_bpe = _Core(mergeable_ranks, special_tokens, pat_str)
            self._fast = True
            # a direct call into the core, without a Python frame; the core
            # cleans lone surrogates itself
            self.encode_ordinary = self._core_bpe.encode_ordinary
        except ValueError:
            from tiktoken import _tiktoken

            self._core_bpe = _tiktoken.CoreBPE(mergeable_ranks, special_tokens, pat_str)
            self._fast = False

    def __repr__(self) -> str:
        return f"<Encoding {self.name!r} (tiktoken-fast)>"

    def encode(
        self,
        text: str,
        *,
        allowed_special: Literal["all"] | AbstractSet[str] = frozenset(),
        disallowed_special: Literal["all"] | Collection[str] = "all",
    ) -> list[int]:
        if self._fast and not allowed_special and disallowed_special == "all":
            tokens = self._core_bpe.encode_strict(text)
            if tokens is not None:
                return tokens
        return super().encode(text, allowed_special=allowed_special, disallowed_special=disallowed_special)

    def count_tokens(
        self,
        text: str,
        *,
        allowed_special: Literal["all"] | AbstractSet[str] = frozenset(),
        disallowed_special: Literal["all"] | Collection[str] = "all",
    ) -> int:
        """len(self.encode(text, ...)) without building the list. Not in tiktoken."""
        if not self._fast:
            return len(self.encode(text, allowed_special=allowed_special, disallowed_special=disallowed_special))
        if not allowed_special and disallowed_special == "all":
            n = self._core_bpe.count_strict(text)
            if n is not None:
                return n
        if allowed_special == "all":
            allowed_special = self.special_tokens_set
        if disallowed_special == "all":
            disallowed_special = self.special_tokens_set - allowed_special
        if disallowed_special:
            if not isinstance(disallowed_special, frozenset):
                disallowed_special = frozenset(disallowed_special)
            if match := _special_token_regex(disallowed_special).search(text):
                raise_disallowed_special_token(match.group())
        return self._core_bpe.count(text, allowed_special)

    def encode_ordinary_batch(self, text: list[str], *, num_threads: int = 8) -> list[list[int]]:
        if self._fast:
            try:
                return self._core_bpe.encode_ordinary_batch(text, num_threads)
            except UnicodeEncodeError:
                pass
        return super().encode_ordinary_batch(text, num_threads=num_threads)

    def encode_batch(
        self,
        text: list[str],
        *,
        num_threads: int = 8,
        allowed_special: Literal["all"] | AbstractSet[str] = set(),  # noqa: B006
        disallowed_special: Literal["all"] | Collection[str] = "all",
    ) -> list[list[int]]:
        if not self._fast:
            return super().encode_batch(
                text, num_threads=num_threads, allowed_special=allowed_special, disallowed_special=disallowed_special
            )
        if allowed_special == "all":
            allowed_special = self.special_tokens_set
        if disallowed_special == "all":
            disallowed_special = self.special_tokens_set - allowed_special
        if disallowed_special:
            if not isinstance(disallowed_special, frozenset):
                disallowed_special = frozenset(disallowed_special)
            search = _special_token_regex(disallowed_special).search
            for t in text:
                if match := search(t):
                    raise_disallowed_special_token(match.group())
        try:
            return self._core_bpe.encode_batch(text, allowed_special, num_threads)
        except UnicodeEncodeError:
            return super().encode_batch(
                text, num_threads=num_threads, allowed_special=allowed_special, disallowed_special=()
            )

    def __getstate__(self) -> object:
        if _ENCODINGS.get(self.name) is self:
            return self.name
        return {
            "name": self.name,
            "pat_str": self._pat_str,
            "mergeable_ranks": self._mergeable_ranks,
            "special_tokens": self._special_tokens,
        }

    def __setstate__(self, value: object) -> None:
        if isinstance(value, str):
            self.__dict__ = get_encoding(value).__dict__
            return
        self.__init__(**value)


_ENCODINGS: dict[str, Encoding] = {}
_LOCK = threading.RLock()


def _constructor_args(name: str) -> dict:
    reg = tiktoken.registry
    find = getattr(reg, "_find_constructors", None)
    if find is not None:
        if reg.ENCODING_CONSTRUCTORS is None:
            find()
        ctor = (reg.ENCODING_CONSTRUCTORS or {}).get(name)
        if ctor is not None:
            return ctor()
    # unknown name: tiktoken raises its own error; a known one we could not
    # reach through the registry is copied from tiktoken's encoding
    enc = reg.get_encoding(name)
    return {
        "name": enc.name,
        "pat_str": enc._pat_str,
        "mergeable_ranks": enc._mergeable_ranks,
        "special_tokens": enc._special_tokens,
    }


def get_encoding(encoding_name: str) -> Encoding:
    """The encoding of that name, as tiktoken.get_encoding, with the fast core."""
    enc = _ENCODINGS.get(encoding_name)
    if enc is not None:
        return enc
    with _LOCK:
        enc = _ENCODINGS.get(encoding_name)
        if enc is None:
            enc = Encoding(**_constructor_args(encoding_name))
            _ENCODINGS[encoding_name] = enc
        return enc


def encoding_for_model(model_name: str) -> Encoding:
    """The encoding a model uses, as tiktoken.encoding_for_model."""
    return get_encoding(encoding_name_for_model(model_name))


_ORIGINAL: dict = {}


def patch() -> None:
    """Makes tiktoken.get_encoding and tiktoken.encoding_for_model (and their
    registry and model module versions) return tiktoken-fast encodings. Code
    that bound those functions before the call keeps the originals."""
    if _ORIGINAL:
        return
    _ORIGINAL.update(
        top_get=tiktoken.get_encoding,
        top_for_model=tiktoken.encoding_for_model,
        reg_get=tiktoken.registry.get_encoding,
        model_get=tiktoken.model.get_encoding,
        model_for_model=tiktoken.model.encoding_for_model,
    )
    tiktoken.get_encoding = get_encoding
    tiktoken.encoding_for_model = encoding_for_model
    tiktoken.registry.get_encoding = get_encoding
    tiktoken.model.get_encoding = get_encoding
    tiktoken.model.encoding_for_model = encoding_for_model


def unpatch() -> None:
    """Undoes patch()."""
    if not _ORIGINAL:
        return
    tiktoken.get_encoding = _ORIGINAL["top_get"]
    tiktoken.encoding_for_model = _ORIGINAL["top_for_model"]
    tiktoken.registry.get_encoding = _ORIGINAL["reg_get"]
    tiktoken.model.get_encoding = _ORIGINAL["model_get"]
    tiktoken.model.encoding_for_model = _ORIGINAL["model_for_model"]
    _ORIGINAL.clear()
