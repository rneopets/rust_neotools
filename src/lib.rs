mod islandmystic;
mod php5random;
mod symol;
use pyo3::{prelude::*, types::PyDateTime};

use islandmystic::IslandMystic;
use php5random::{PyPhp5MtRandom, PyPhp5Random};
use symol::Symol;

/// Reads year, month and day from a datetime via attribute access,
/// since the typed accessors are unavailable under the stable ABI.
pub(crate) fn ymd(dt: &Bound<'_, PyDateTime>) -> PyResult<(i32, u8, u8)> {
    Ok((
        dt.getattr("year")?.extract()?,
        dt.getattr("month")?.extract()?,
        dt.getattr("day")?.extract()?,
    ))
}

#[pymodule]
#[pyo3(name = "rust_neotools")]
fn rust_neotools(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<IslandMystic>()?;
    m.add_class::<Symol>()?;
    m.add_class::<PyPhp5Random>()?;
    m.add_class::<PyPhp5MtRandom>()?;

    Ok(())
}
