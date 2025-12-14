// Autopilot - Player

// ============================================================================================== //
// Imports

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::tile::*;
use super::super::settings::*;

// ============================================================================================== //
// Constants

// ============================================================================================== //

#[derive(Clone)]
#[derive(Component)]
pub struct Chunk {
	pub tiles: Vec<Tile>,		// List of tiles, in a 100x100 grid
	pub quadrant_number: u64,	// The quadrant this chunk is located within
	pub chunk_number: u64		// This chunk's id number, via diagonal ordering within a quadrant
}

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

	/// This function changes the Terrain type of an existing Tile object.
	pub fn update_tile_terrain(&mut self, x_chunk: usize, y_chunk: usize, new_terrain: Terrain) {
		let index: usize = CHUNK_SIZE * y_chunk + x_chunk;
		self.tiles[index].terrain = new_terrain;
	}
}

pub fn spawn_chunk(
	mut commands: Commands,
	tile_textures: Res<TileTextures>,
	x_map_offset: i64,
	y_map_offset: i64,
	chunk_number: u64,
	quadrant_number: u64
) {
//	const WATER_PERCENT: f32 = 20.0;
//	const SAND_PERCENT: f32 = 10.0;
//	const GRASS_START: f32 = WATER_PERCENT + SAND_PERCENT;

//	let mut rng = rand::rng();

	// Set up map dimensions
	let map_size = TilemapSize{x: CHUNK_SIZE as u32, y: CHUNK_SIZE as u32};
	let grid_size = TilemapGridSize{x: TILE_WIDTH, y: TILE_HEIGHT};
	let tilemap_size = TilemapTileSize{x: TILE_WIDTH, y: TILE_HEIGHT};
	let map_type = TilemapType::Isometric(IsoCoordSystem::Diamond);

	let mut tile_storage: TileStorage = TileStorage::empty(map_size);
	let tilemap_entity: Entity = commands.spawn_empty().id();

	// Create a new Chunk
	let mut chunk: Chunk = Chunk::new(chunk_number, quadrant_number);

	for x in 0..CHUNK_SIZE {
		for y in 0..CHUNK_SIZE {
			let texture_index = 0;
			let terrain_type = Terrain::Grass;

			let tile = Tile {
				x_map: x as i64 + x_map_offset,
				y_map: y as i64 + y_map_offset,
				quadrant: quadrant_number,
				terrain: terrain_type,
			};

			// Add Tile to Chunk
			chunk.tiles.push(tile.clone());

			// Bevy ECS's TilePos
			let tile_pos = TilePos{x: x.try_into().unwrap(), y: y.try_into().unwrap()};

			let tile_entity = commands.spawn((
				TileBundle {
					position: tile_pos,
					tilemap_id: TilemapId(tilemap_entity),
					texture_index: TileTextureIndex(texture_index),
					color: TileColor(Color::WHITE),
					..Default::default()
				},
				tile
			)).id();

			tile_storage.set(&tile_pos, tile_entity);
		}
	}

	commands.entity(tilemap_entity).insert(TilemapBundle {
		grid_size,
		size: map_size,
		tile_size: tilemap_size,
		texture: tile_textures.textures.clone(),
		storage: tile_storage,
		anchor: TilemapAnchor::Center,
		map_type,
		..Default::default()
	});

	commands.entity(tilemap_entity).insert(chunk);
}