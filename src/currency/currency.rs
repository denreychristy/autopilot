// AutoPilot - Rust - Currency

#[allow(dead_code)]

// ============================================================================================== //

use pyo3::prelude::*;

// ============================================================================================== //

#[pyclass]
pub struct Currency {
	name: String,			// like "dollar" or "krone"
	name_plural: String,	// like "dollars" or "kroner"
	supply: f64,			// the total amount of money that has been created
}

#[pymethods]
impl Currency {
	#[new]
	pub fn new(name: &str, name_plural: &str, supply: f64) -> Self {
		Currency {
			name: name.to_string(),
			name_plural: name_plural.to_string(),
			supply: supply
		}
	}

	// Getter for 'name'
	pub fn get_name(&self) -> &str {
		&self.name 
	}

	// Getter for 'name_plural'
	pub fn get_plural(&self) -> &str {
		&self.name_plural 
	}

	// Getter for 'supply'
	pub fn get_supply(&self) -> f64 {
		self.supply
	}

	pub fn add_money(&mut self, amount: f64) {
		self.supply += amount
	}
}