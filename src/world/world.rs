// AutoPilot - Rust - World

#[allow(dead_code)]

// ============================================================================================== //

use pyo3::prelude::*;

use super::tile::Tile;

// ============================================================================================== //

// quadrant 3 = quadrant 4
// -x, -y     = x, -y
// ========== = ==========
// -x, y      = x, y
// quadrant 2 = quadrant 1

#[pyclass]
#[derive(Debug, Clone)]
pub struct World {
	pub quadrant_1: Vec<Vec<Tile>>,
	pub quadrant_2: Vec<Vec<Tile>>,
	pub quadrant_3: Vec<Vec<Tile>>,
	pub quadrant_4: Vec<Vec<Tile>>
}

#[pymethods]
impl World {
	#[new]
	pub fn new() -> Self {
		let mut quadrant_1: Vec<Vec<Tile>> = Vec::with_capacity(1_000);
		for _ in 0..10_000 {
			let row: Vec<Tile> = (0..1_000)
				.map(|_| Tile::new())
				.collect();
			quadrant_1.push(row);
		}

		let mut quadrant_2: Vec<Vec<Tile>> = Vec::with_capacity(1_000);
		for _ in 0..10_000 {
			let row: Vec<Tile> = (0..1_000)
				.map(|_| Tile::new())
				.collect();
			quadrant_2.push(row);
		}

		let mut quadrant_3: Vec<Vec<Tile>> = Vec::with_capacity(1_000);
		for _ in 0..10_000 {
			let row: Vec<Tile> = (0..1_000)
				.map(|_| Tile::new())
				.collect();
			quadrant_3.push(row);
		}

		let mut quadrant_4: Vec<Vec<Tile>> = Vec::with_capacity(1_000);
		for _ in 0..1_000 {
			let row: Vec<Tile> = (0..10_000)
				.map(|_| Tile::new())
				.collect();
			quadrant_4.push(row);
		}

		World {
			quadrant_1,
			quadrant_2,
			quadrant_3,
			quadrant_4
		}
	}

	pub fn get(&self, x: i64, y: i64) -> Tile {
		if x >= 0 && y >= 0 {
			let x_index: usize = x as usize;
			let y_index: usize = y as usize;
			return self.quadrant_1[x_index][y_index].clone()
		}
		else if x < 0 && y >= 0 {
			let x_index: usize = (-1 * x) as usize;
			let y_index: usize = y as usize;
			return self.quadrant_2[x_index][y_index].clone()
		}
		else if x < 0 && y < 0 {
			let x_index: usize = (-1 * x) as usize;
			let y_index: usize = (-1 * y) as usize;
			return self.quadrant_3[x_index][y_index].clone()
		}
		else {
			let x_index: usize = x as usize;
			let y_index: usize = (-1 * y) as usize;
			return self.quadrant_4[x_index][y_index].clone()
		}
	}

	pub fn get_content(&self, x: i64, y: i64) -> Option<String> {
		self.get(x, y).content
	}
}