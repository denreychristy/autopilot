// AutoPilot - Rust - Tile

#[allow(dead_code)]

// ============================================================================================== //

use pyo3::prelude::*;

// ============================================================================================== //

#[pyclass]
#[derive(Debug, Clone)]
pub struct Tile {
	#[pyo3(get)]
	pub content: Option<String>
}

#[pymethods]
impl Tile {
	#[new]
	pub fn new() -> Self {
		Tile {
			content: None
		}
	}
}