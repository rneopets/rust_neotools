mod islandmystic;
mod php5random;
mod symol;
use pyo3::prelude::*;

use islandmystic::IslandMystic;
use php5random::{PyPhp5MtRandom, PyPhp5Random};
use symol::Symol;

#[pymodule]
#[pyo3(name = "rust_neotools")]
fn rust_neotools(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<IslandMystic>()?;
    m.add_class::<Symol>()?;
    m.add_class::<PyPhp5Random>()?;
    m.add_class::<PyPhp5MtRandom>()?;

    Ok(())
}
