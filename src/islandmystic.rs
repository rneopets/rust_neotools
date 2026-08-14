use chrono::{Datelike, NaiveDate};
use itertools::iproduct;
use pyo3::{
    prelude::*,
    types::{PyDateAccess, PyDateTime, PyTzInfoAccess},
};
use rayon::iter::{ParallelBridge, ParallelIterator};

use php5rand::Php5Random;

const CHARS: &str = "abcdefghijklmnopqrstuvwxyz_0123456789";

#[pyclass]
pub struct IslandMystic;

impl IslandMystic {
    /// Creates a new Php5Random instance with the given username and date.
    /// This isn't really to be used outside of this module.
    fn new_rng(username: &str, year: i32, month: u8, day: u8) -> Php5Random {
        let mut username_ord = username
            .chars()
            .take(4)
            .map(|c| c as u32)
            .collect::<Vec<u32>>();

        if username_ord.len() < 4 {
            username_ord.resize(4, 0);
        }

        let yyyymmdd: u32 = year as u32 * 10000 + month as u32 * 100 + day as u32;

        let magic_number: u32 = yyyymmdd
            + 287234 * username_ord[0]
            + 71 * username_ord[1]
            + 97 * username_ord[2]
            + 1045 * username_ord[3];

        Php5Random::new(magic_number)
    }

    /// Checks if the given username has an avatar on the given date, in English.
    #[inline]
    fn check_rust(username: &str, year: i32, month: u8, day: u8) -> bool {
        let mut rng: Php5Random = IslandMystic::new_rng(username, year, month, day);

        if rng.rand() % 3 == 2 {
            if rng.rand() % 2 == 1 {
                rng.rand();
                rng.rand();
                rng.rand();
                rng.rand();
                let result = rng.rand() % 23;
                return (result > 1 && result < 5) && (rng.rand() % 20 == 11);
            }
        } else if rng.rand().is_multiple_of(3) {
            rng.rand();
            rng.rand();
            rng.rand();
            rng.rand();
            rng.rand();
            rng.rand();
            return rng.rand() % 20 == 11;
        }

        false
    }

    /// Checks if the given username has an avatar on the given date, in non-English.
    /// I'm not entirely sure why non-English folk are so much less likely to get an avatar.
    #[inline]
    fn check_non_english_rust(username: &str, year: i32, month: u8, day: u8) -> bool {
        let mut rng: Php5Random = IslandMystic::new_rng(username, year, month, day);

        rng.rand().is_multiple_of(920)
    }

    pub fn brute_force_day_rust(year: i32, month: u8, day: u8, english: bool) -> Vec<String> {
        let func = if english {
            IslandMystic::check_rust
        } else {
            IslandMystic::check_non_english_rust
        };

        let char_combinations = iproduct!(CHARS.chars(), CHARS.chars(), CHARS.chars());

        let mut usernames: Vec<String> = char_combinations
            .par_bridge() // Parallelize processing
            .filter_map(|(c0, c1, c2)| {
                let username = [c0, c1, c2].iter().collect::<String>();
                if func(&username, year, month, day) {
                    Some(username)
                } else {
                    None
                }
            })
            .collect();

        let longer_char_combinations =
            iproduct!(CHARS.chars(), CHARS.chars(), CHARS.chars(), CHARS.chars());

        usernames.extend(
            longer_char_combinations
                .par_bridge() // Parallelize processing
                .filter_map(|(c0, c1, c2, c3)| {
                    let username = [c0, c1, c2, c3].iter().collect::<String>();
                    if func(&username, year, month, day) {
                        Some(username)
                    } else {
                        None
                    }
                })
                .collect::<Vec<String>>(),
        );

        //  sort usernames alphabetically
        usernames.sort_unstable();

        usernames
    }

    pub fn brute_force_user_rust(
        username: &str,
        year: i32,
        month: u8,
        day: u8,
        step: i64,
        english: bool,
    ) -> Option<NaiveDate> {
        // safeguard against infinite loops
        if step == 0 {
            return None;
        }

        let mut chrono_dt = NaiveDate::from_ymd_opt(year, month as u32, day as u32);

        let func = if english {
            IslandMystic::check_rust
        } else {
            IslandMystic::check_non_english_rust
        };

        let duration = chrono::Duration::days(step);

        while let Some(new_dt) = chrono_dt {
            let is_avatar_date = func(
                username,
                new_dt.year(),
                new_dt.month() as u8,
                new_dt.day() as u8,
            );

            match is_avatar_date {
                true => break,
                false => chrono_dt = new_dt.checked_add_signed(duration),
            }
        }

        chrono_dt
    }
}

// just a wrapper around the rust functions, so they can be called from python
#[pymethods]
impl IslandMystic {
    #[staticmethod]
    pub fn check(dt: &Bound<PyDateTime>, username: &str) -> bool {
        IslandMystic::check_rust(username, dt.get_year(), dt.get_month(), dt.get_day())
    }

    #[staticmethod]
    pub fn check_non_english(dt: &Bound<PyDateTime>, username: &str) -> bool {
        IslandMystic::check_non_english_rust(username, dt.get_year(), dt.get_month(), dt.get_day())
    }

    #[staticmethod]
    pub fn brute_force_day(dt: &Bound<PyDateTime>, english: bool) -> Vec<String> {
        let year = dt.get_year();
        let month = dt.get_month();
        let day = dt.get_day();

        IslandMystic::brute_force_day_rust(year, month, day, english)
    }

    #[staticmethod]
    pub fn brute_force_user<'a>(
        dt: &Bound<'a, PyDateTime>,
        username: &str,
        step: i64,
        english: bool,
    ) -> Option<Bound<'a, PyDateTime>> {
        let chrono_dt = IslandMystic::brute_force_user_rust(
            username,
            dt.get_year(),
            dt.get_month(),
            dt.get_day(),
            step,
            english,
        );

        if let Some(finished) = chrono_dt {
            let new_dt = PyDateTime::new(
                dt.py(),
                finished.year(),
                finished.month() as u8,
                finished.day() as u8,
                0,
                0,
                0,
                0,
                dt.get_tzinfo().as_ref(),
            );

            if let Ok(new_dt) = new_dt {
                return Some(new_dt);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mystic_english() {
        let valid = IslandMystic::check_rust("diceroll123", 2022, 10, 30);
        assert!(valid);
    }

    #[test]
    fn test_mystic_english_short() {
        let valid = IslandMystic::check_rust("yee", 2023, 5, 29);
        assert!(valid);
    }

    #[test]
    fn test_mystic_english_wrong() {
        let valid = IslandMystic::check_rust("diceroll123", 2023, 3, 10);
        assert!(!valid);
    }

    #[test]
    fn test_mystic_non_english() {
        let valid = IslandMystic::check_rust("diceroll123", 2030, 6, 21);
        assert!(!valid);
    }

    #[test]
    fn test_mystic_non_english_wrong() {
        let valid = IslandMystic::check_rust("diceroll123", 2023, 3, 10);
        assert!(!valid);
    }

    #[test]
    fn test_mystic_out_of_bounds_brute_force_user() {
        // I say "out of bounds" because Python currently doesn't go above year 9999,
        // but Rust does. So we test for that anyways!
        let date = IslandMystic::brute_force_user_rust("diceroll123", 9999, 10, 31, 1, false);
        assert!(date == NaiveDate::from_ymd_opt(10001, 1, 6));
    }

    #[test]
    fn test_mystic_brute_force_user() {
        let date = IslandMystic::brute_force_user_rust("diceroll123", 2023, 3, 31, 1, true);
        assert!(date == NaiveDate::from_ymd_opt(2023, 4, 2));
    }

    #[test]
    fn test_mystic_brute_force_user_non_english() {
        let usernames = IslandMystic::brute_force_day_rust(2023, 1, 3, true);
        assert!(!usernames.is_empty());
    }
}
