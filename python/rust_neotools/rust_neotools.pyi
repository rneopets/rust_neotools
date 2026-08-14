from datetime import datetime

class Symol:
    @staticmethod
    def get_minute(date: datetime, /) -> int:
        """Returns the Symol minute of the day for the given datetime."""
        ...
    @staticmethod
    def get_window(date: datetime, /) -> list[int]:
        """Returns the window of Symol minutes of the day for the given datetime."""
        ...

class IslandMystic:
    @staticmethod
    def check(date: datetime, username: str) -> bool:
        """Checks if the username can get the Island Mystic avatar on this date."""
        ...
    @staticmethod
    def check_non_english(date: datetime, username: str) -> bool:
        """Checks if the username can get the Island Mystic avatar on this date."""
        ...
    @staticmethod
    def brute_force_day(date: datetime, english: bool) -> list[str]:
        """Brute forces all username prefixes for the given date."""
        ...
    @staticmethod
    def brute_force_user(
        date: datetime, username: str, step: int, english: bool
    ) -> datetime | None:
        """Brute forces all dates for the given username, for the language and step direction of time given."""
        ...

class Php5Random:
    """PHP5's `rand()`/`srand()` (on Linux, backed by glibc's `random()`)."""

    def __init__(self, seed: int, /) -> None: ...
    def rand(self) -> int:
        """Returns the next random number in the sequence."""
        ...
    def rand_range(self, min: int, max: int, /) -> int:
        """Returns the next random number in the sequence, scaled to the given range."""
        ...

class Php5MtRandom:
    """PHP5's `mt_rand()`/`mt_srand()` (a genuine Mersenne Twister, MT19937)."""

    def __init__(self, seed: int, /) -> None: ...
    def rand(self) -> int:
        """Returns the next random number in the sequence."""
        ...
    def rand_range(self, min: int, max: int, /) -> int:
        """Returns the next random number in the sequence, scaled to the given range."""
        ...
