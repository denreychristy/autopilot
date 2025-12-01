// Autopilot - Modules - World - World

// ============================================================================================== //

use bevy::prelude::*;

use super::chunk::Chunk;
use super::tile::Tile;

// ============================================================================================== //

#[derive(Component)]
pub struct World {
	pub quadrant_0: Vec<Chunk>,
	pub quadrant_1: Vec<Chunk>,
	pub quadrant_2: Vec<Chunk>,
	pub quadrant_3: Vec<Chunk>
}

pub fn get_tile(world_query: Query<&World>, mut x_tile: i64, mut y_tile: i64) -> Option<Tile> {
	let world = world_query[0].unwrap();

	let x_positive: bool = x_tile >= 0;
	let y_positive: bool = y_tile >= 0;
	if !x_positive { x_tile += 1; }
	if !y_positive { y_tile += 1; }

	let x_within_chunk: usize = coord_within_chunk(x_tile);
	let y_within_chunk: usize = coord_within_chunk(y_tile);
	let x_chunk: u64 = (x_tile.abs() / 100) as u64;
	let y_chunk: u64 = (y_tile.abs() / 100) as u64;
	let chunk_number: usize = get_chunk(x_chunk, y_chunk) as usize;

	match (x_positive, y_positive) {
		(true, true) => { // quadrant 0
			while chunk_number >= self.quadrant_0.len() {
				self.quadrant_0.push(Chunk::new());
			}
			Some(self.quadrant_0[chunk_number].get_tile(x_within_chunk, y_within_chunk))
		}
		(false, true) => { // quadrant 1
			while chunk_number >= self.quadrant_1.len() {
				self.quadrant_1.push(Chunk::new());
			}
			Some(self.quadrant_1[chunk_number].get_tile(x_within_chunk, y_within_chunk))
		}
		(false, false) => { // quadrant 2
			while chunk_number >= self.quadrant_2.len() {
				self.quadrant_2.push(Chunk::new());
			}
			Some(self.quadrant_2[chunk_number].get_tile(x_within_chunk, y_within_chunk))
		}
		(true, false) => { // quadrant 3
			while chunk_number >= self.quadrant_3.len() {
				self.quadrant_3.push(Chunk::new());
			}
			Some(self.quadrant_3[chunk_number].get_tile(x_within_chunk, y_within_chunk))
		}
	}
}