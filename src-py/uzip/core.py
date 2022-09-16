from __future__ import annotations

from typing import AnyStr

import python_minifier

from uzip import uzip

__all__ = ['pack']


def pack(s: AnyStr, minify: bool = False) -> str:
    """Packs a string or file-like object into compressed unicode"""
    if isinstance(s, str):
        s = s.encode('utf-8')
    if minify:
        s = python_minifier.minify(s)
        s = s.encode('utf-8')
    return uzip.b2048encode(uzip.compress(s))
