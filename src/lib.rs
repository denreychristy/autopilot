// AutoPilot - Rust

#[allow(dead_code)]

// ============================================================================================== //

use pyo3::prelude::*;

mod currency;
mod non_player_character;
mod world;

use currency::currency::Currency;
use non_player_character::non_player_character::NonPlayerCharacter;
use world::tile::Tile;
use world::world::World;

// ============================================================================================== //

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

// ============================================================================================== //

/// A Python module implemented in Rust.
#[pymodule]
fn rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;

	// Classes
	m.add_class::<Currency>()?;
	m.add_class::<NonPlayerCharacter>()?;
	m.add_class::<Tile>()?;
	m.add_class::<World>()?;

    Ok(())
}