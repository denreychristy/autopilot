// AutoPilot - Rust - Non Player Character

#[allow(dead_code)]

// ============================================================================================== //

use std::sync::atomic::{AtomicU64, Ordering};

use pyo3::prelude::*;

// ============================================================================================== //

static COUNTER: AtomicU64 = AtomicU64::new(0);

// ============================================================================================== //

#[pyclass]
pub struct NonPlayerCharacter {
	id: u64,
	currency: f64
}

// ============================================================================================== //

#[pymethods]
impl NonPlayerCharacter {
	#[new]
	pub fn new() -> Self {
		let new_id: u64 = COUNTER.fetch_add(1, Ordering::SeqCst);
		NonPlayerCharacter {
			id: new_id,
			currency: 0.0
		}
	}

	// ================================================== //
	// Getter Methods //

	pub fn get_id(&self) -> u64 {
		self.id
	}

	pub fn get_currency(&self) -> f64 {
		self.currency
	}

	// ================================================== //
	// Other Methods //
}