// AutoPilot - Modules - World - Chunk

// ============================================================================================== //

use bevy::prelude::*;

use super::tile::Tile;

// ============================================================================================== //
// Get Chunk Number From Chunk Coordinates

pub fn get_chunk_x0(x: u64) -> u64 {
	if x == 0 {
		0
	}
	else {
		get_chunk_x0(x - 1) + x
	}
}

pub fn get_chunk(x: u64, y: u64) -> u64 {
	if y == 0 {
		get_chunk_x0(x)
	}
	else {
		get_chunk(x, y - 1) + x + y + 1
	}
}

// ============================================================================================== //
// Get the x & y coordinates within a chunk given the world x & y coordinates

pub fn coord_within_chunk(c: i64) -> usize {
	if c >= 0 {
		(c % 100) as usize
	}
	else {
		let c_mod: i64 = c.abs() % 100;
		if c_mod == 0 {
			0 as usize
		}
		else {
			(100 - c_mod) as usize
		}
	}
}

// ============================================================================================== //

#[derive(Debug, Clone)]
pub struct Chunk {
	pub tiles: Vec<Tile> // 100 x 100
}

impl Chunk {
	pub fn new() -> Self {
		let chunk_size: u16 = 100 * 100;
		Chunk {
			tiles: (0 .. chunk_size)
        		.map(|_| Tile::new())
        		.collect()
		}
	}

	pub fn get_tile(&self, x_within_chunk: usize, y_within_chunk: usize) -> Tile {
		let index: usize = (100 * y_within_chunk) + x_within_chunk;
		self.tiles[index].clone()
	}

	pub fn set_tile_visibile_and_discovered(&mut self, x_within_chunk: usize,
		y_within_chunk: usize, visible: bool, discovered: bool) {
		
		let index: usize = (100 * y_within_chunk) + x_within_chunk;
		self.tiles[index].visible = visible;
		self.tiles[index].discovered = discovered; 
	}
}