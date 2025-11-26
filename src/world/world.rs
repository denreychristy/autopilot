// AutoPilot - Rust - World

#[allow(dead_code)]

// ============================================================================================== //

use pyo3::prelude::*;

use super::tile::Tile;

// ============================================================================================== //
// Get Chunk Number From Chunk Coordinates

fn get_chunk_x0(x: u64) -> u64 {
	if x == 0 {
		0
	}
	else {
		get_chunk_x0(x - 1) + x
	}
}

fn get_chunk(x: u64, y: u64) -> u64 {
	if y == 0 {
		get_chunk_x0(x)
	}
	else {
		get_chunk(x, y - 1) + x + y + 1
	}
}

// ============================================================================================== //

#[pyclass]
#[derive(Debug, Clone)]
pub struct Chunk {
	pub tiles: Vec<Tile> // 100 x 100
}

#[pymethods]
impl Chunk {
	#[new]
	pub fn new() -> Self {
		let chunk_size: u16 = 100 * 100;
		Chunk {
			tiles: (0 .. chunk_size)
        		.map(|_| Tile::new())
        		.collect()
		}
	}

	pub fn get(&self, x_within_chunk: usize, y_within_chunk: usize) -> Tile {
		let index: usize = (100 * y_within_chunk) + x_within_chunk;
		self.tiles[index].clone()
	}
}

// ============================================================================================== //

// quadrant 2 = quadrant 3
// -x, -y     = x, -y
// ========== = ==========
// -x, y      = x, y
// quadrant 1 = quadrant 0

#[pyclass]
#[derive(Debug, Clone)]
pub struct World {
	pub quadrant_0: Vec<Chunk>,
	pub quadrant_1: Vec<Chunk>,
	pub quadrant_2: Vec<Chunk>,
	pub quadrant_3: Vec<Chunk>
}

#[pymethods]
impl World {
	#[new]
	pub fn new() -> Self {
		World {
			quadrant_0: vec![Chunk::new()],
			quadrant_1: vec![Chunk::new()],
			quadrant_2: vec![Chunk::new()],
			quadrant_3: vec![Chunk::new()]
		}
	}

	pub fn get(&mut self, x_world: i64, y_world: i64) -> Option<Tile> {
		// this part is wrong
		let x_within_chunk: usize = (100 - (x_world.abs() % 100)) as usize;
		let y_within_chunk: usize = (100 - (y_world.abs() % 100)) as usize;
		//
		let x_chunk: u64 = (x_world.abs() / 100) as u64;
		let y_chunk: u64 = (y_world.abs() / 100) as u64;
		println!("x_chunk: {}, y_chunk: {}", x_chunk, y_chunk);
		let chunk_number: usize = get_chunk(x_chunk, y_chunk) as usize;

		println!("Chunk number: {}", chunk_number);

		let x_positive: bool = x_world >= 0;
		let y_positive: bool = y_world >= 0;
		println!("{}, {}", x_positive, y_positive);

		match (x_positive, y_positive) {
			(true, true) => { // quadrant 0
				while chunk_number >= self.quadrant_0.len() {
					self.quadrant_0.push(Chunk::new());
				}
				Some(self.quadrant_0[chunk_number].get(x_within_chunk, y_within_chunk))
			}
			(false, true) => { // quadrant 1
				while chunk_number >= self.quadrant_1.len() {
					self.quadrant_1.push(Chunk::new());
				}
				Some(self.quadrant_1[chunk_number].get(x_within_chunk, y_within_chunk))
			}
			(false, false) => { // quadrant 2
				while chunk_number >= self.quadrant_2.len() {
					self.quadrant_2.push(Chunk::new());
				}
				Some(self.quadrant_2[chunk_number].get(x_within_chunk, y_within_chunk))
			}
			(true, false) => { // quadrant 3
				while chunk_number >= self.quadrant_3.len() {
					self.quadrant_3.push(Chunk::new());
				}
				Some(self.quadrant_3[chunk_number].get(x_within_chunk, y_within_chunk))
			}
		}
	}
}