# rust_neotools

A high-performance Python library (written in Rust) for Neopets-related calculations, focusing on Island Mystic avatar eligibility and Symol Hole timing predictions.

## Installation

*Install uv from [astral-sh/uv](https://github.com/astral-sh/uv)*

```bash
uv pip install rust_neotools
```

## Features

- **Blazing fast**: Written in Rust with parallel processing for brute-force operations
- **Zero dependencies**: Only requires `datetime` from Python's standard library
- **Type-safe**: Fully typed with `.pyi` stub files included

## API Reference

### IslandMystic

The Island Mystic class provides methods to check avatar eligibility and brute-force search for usernames or dates.

#### `IslandMystic.check(date, username)`

Check if a username is eligible for an Island Mystic avatar on a specific date (English language).

**Parameters:**
- `date` (datetime): The date to check
- `username` (str): The Neopets username to check

**Returns:** `bool` - True if eligible, False otherwise

**Example:**
```python
from datetime import datetime
from rust_neotools import IslandMystic

date = datetime(2022, 10, 30)
username = "diceroll123"

if IslandMystic.check(date, username):
    print(f"{username} gets an avatar on {date}!")
```

#### `IslandMystic.check_non_english(date, username)`

Check if a username is eligible for an Island Mystic avatar on a specific date (non-English language).

**Parameters:**
- `date` (datetime): The date to check
- `username` (str): The Neopets username to check

**Returns:** `bool` - True if eligible, False otherwise

**Example:**
```python
from datetime import datetime
from rust_neotools import IslandMystic

date = datetime(2030, 6, 21)
username = "diceroll123"

if IslandMystic.check_non_english(date, username):
    print(f"{username} gets an avatar on {date}!")
```

#### `IslandMystic.brute_force_day(date, english)`

Find all 3-4 character usernames that are eligible for an Island Mystic avatar on a specific date.

**Parameters:**
- `date` (datetime): The date to search for
- `english` (bool): True for English language, False for non-English

**Returns:** `list[str]` - Sorted list of eligible usernames

**Example:**
```python
from datetime import datetime
from rust_neotools import IslandMystic

date = datetime(2023, 1, 3)
usernames = IslandMystic.brute_force_day(date, english=True)
print(f"Found {len(usernames)} eligible usernames")
print(f"First few: {usernames[:5]}")
```

**Note:** This method tests all combinations of 3-4 character usernames (alphanumeric + underscore) and uses parallel processing for speed. *It may use up your entire CPU and take several seconds to complete.* 😂

#### `IslandMystic.brute_force_user(date, username, step, english)`

Find the next date when a specific username will be eligible for an Island Mystic avatar.

**Parameters:**
- `date` (datetime): The starting date for the search
- `username` (str): The Neopets username to search for
- `step` (int): Number of days to increment (positive for forward, negative for backward)
  - Generally this is expected to be either 1 or -1.
- `english` (bool): True for English language, False for non-English

**Returns:** `datetime | None` - The next eligible date, or None if not found within datetime bounds

**Example:**
```python
from datetime import datetime
from rust_neotools import IslandMystic

start_date = datetime(2023, 3, 31)
username = "diceroll123"

next_date = IslandMystic.brute_force_user(start_date, username, step=1, english=True)
if next_date:
    print(f"Next avatar date for {username}: {next_date}")
```

---

### Symol

The Symol class provides methods to determine the Symol Hole time window for any given date.

#### `Symol.get_minute(date)`

Get the starting minute of the Symol Hole window for a specific date.

**Parameters:**
- `date` (datetime): The date to check

**Returns:** `int` - The starting minute (0-59), or 60 for skip days

**Example:**
```python
from datetime import datetime
from rust_neotools import Symol

date = datetime(2023, 3, 10)
minute = Symol.get_minute(date)
print(f"Symol window starts at minute {minute}")
```

**Note:** A return value of 60 indicates a "skip day" where the Symol event is not available.

#### `Symol.get_window(date)`

Get the full Symol Hole time window (up to 4 minutes) for a specific date.

**Parameters:**
- `date` (datetime): The date to check

**Returns:** `list[int]` - List of valid minutes (empty list for skip days)

**Example:**
```python
from datetime import datetime
from rust_neotools import Symol

date = datetime(2023, 3, 10)
window = Symol.get_window(date)  # [9, 10, 11, 12]
print(f"Valid minutes: {window}")
```

**Note:** The window may contain fewer than 4 minutes if it would extend past minute 59. Skip days return an empty list.

---

## Development

### Building from source

```bash
# Install maturin and pytest
uv pip install maturin pytest

# Build and install locally
maturin develop

# Run tests
cargo test
pytest
```

## Links

- [php5rand](https://github.com/diceroll123/php5rand) - the PHP5 `rand()`/`srand()` and `mt_rand()`/`mt_srand()` reimplementations this project depends on

## License

This project is licensed under the MIT License - see the LICENSE file for details.
