__title__ = "rust_neotools"
__author__ = "diceroll123"
__license__ = "MIT"
__copyright__ = "Copyright 2023-present diceroll123"

from .rust_neotools import IslandMystic, Php5MtRandom, Php5Random, Symol  # noqa: F401

try:
    from importlib.metadata import version

    __version__ = version("rust_neotools")
except Exception:
    __version__ = "unknown"

__all__ = (
    "IslandMystic",
    "Php5MtRandom",
    "Php5Random",
    "Symol",
)
