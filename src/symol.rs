use chrono::{Datelike, NaiveDate};
use pyo3::{
    prelude::*,
    types::{PyDateAccess, PyDateTime},
};

use php5rand::Php5Random;

#[pyclass]
pub struct Symol;

impl Symol {
    /// Returns the first minute that the given date's Symol window is valid for.
    /// Returns 60 on skip-days.
    pub fn get_minute_rust(year: i32, month: u8, day: u8) -> i8 {
        let chrono_dt = NaiveDate::from_ymd_opt(year, month as u32, day as u32).unwrap();

        let seed = year as u32 * 365 + chrono_dt.ordinal0();

        let mut phpr = Php5Random::new(seed);
        phpr.rand_range(1, 60) as i8
    }

    /// Returns a vector of up to 4 minutes that the given date's Symol window is valid for.
    /// The vector can be empty on skip-days, which means the minute is 60.
    pub fn get_window_rust(year: i32, month: u8, day: u8) -> Vec<i8> {
        let minute = Symol::get_minute_rust(year, month, day);

        (minute..minute + 4)
            .filter(|m| m <= &59)
            .collect::<Vec<i8>>()
    }
}

#[pymethods]
impl Symol {
    #[staticmethod]
    fn get_minute(date: &Bound<PyDateTime>) -> i8 {
        Symol::get_minute_rust(date.get_year(), date.get_month(), date.get_day())
    }
    #[staticmethod]
    fn get_window(date: &Bound<PyDateTime>) -> Vec<i8> {
        Symol::get_window_rust(date.get_year(), date.get_month(), date.get_day())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symol_skip_day() {
        let minute = Symol::get_minute_rust(2022, 9, 19);
        assert!(minute == 60)
    }

    #[test]
    fn test_symol() {
        let minute = Symol::get_minute_rust(2023, 3, 10);
        assert!(minute == 9)
    }

    #[test]
    fn test_symol_window() {
        let window = Symol::get_window_rust(2023, 3, 10);
        assert!(window == vec![9, 10, 11, 12])
    }

    #[test]
    fn test_symol_window_capped() {
        let window = Symol::get_window_rust(2023, 4, 5);
        assert!(window == vec![58, 59])
    }

    #[test]
    fn test_symol_window_skip_day_capped() {
        let window = Symol::get_window_rust(2022, 9, 19);
        assert!(window.is_empty())
    }
}
