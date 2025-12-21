// Autopilot - Player

// ============================================================================================== //
// Imports

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::tile::*;
use super::super::settings::*;

// ============================================================================================== //
// struct Chunk

#[derive(Clone)]
#[derive(Component)]
pub struct Chunk {
	pub tiles: Vec<Tile>,		// List of tiles, in a 100x100 grid
	pub quadrant_number: u64,	// The quadrant this chunk is located within
	pub chunk_number: u64		// This chunk's id number, via diagonal ordering within a quadrant
}

// ============================================================================================== //
// impl Chunk

impl Chunk {
	pub fn new(chunk_number: u64, quadrant_number: u64) -> Self {
		let tiles: Vec<Tile> = Vec::with_capacity(CHUNK_SIZE * CHUNK_SIZE);
		Chunk {
			tiles,
			chunk_number,
			quadrant_number
		}
	}

	/// This function returns a mutable reference to the Tile object located at the
	/// (x_chunk, y_chunk) coordinate pair *within the chunk itself*, as opposed to the global map
	/// (x, y) coordinate pair.
	pub fn get_tile(&mut self, x_chunk: usize, y_chunk: usize) -> &mut Tile {
		let index: usize = CHUNK_SIZE * y_chunk + x_chunk;
		return &mut self.tiles[index];
	}
}