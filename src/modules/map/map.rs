// Autopilot - Map

// ============================================================================================== //
// Imports

use num::integer::div_floor;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::super::map::chunk::*;
use super::super::map::tile::*;
use super::super::settings::*;

// ============================================================================================== //
// Chunk Diagonal Numbering Helper Functions

pub fn get_chunk_x(x: u64) -> u64 {
	match x {
		0 => {
			return 0;
		}
		_ => {
			return get_chunk_x(x - 1) + x;
		}
	}
}

pub fn get_chunk_number(x: u64, y: u64) -> u64 {
	match y {
		0 => {
			return get_chunk_x(x);
		}
		_ => {
			return get_chunk_number(x, y-1) + x + y;
		}
	}
}

pub fn get_x_y_from_chunk(chunk: usize) -> (u64, u64) {
	if chunk == 0 {
		return (0, 0);
	}

	// 0 -> (0, 0)
	// 1 -> (1, 0)
	// 2 -> (0, 1)
	// 3 -> (2, 0)
	// 4 -> (1, 1)
	// 5 -> (0, 2)
	let mut x: u64 = 0;
	let mut y: u64 = 0;
	let mut level: u64 = 0;
	let mut counter: u64 = 0;

	while counter <= chunk as u64 {
		if x == 0 {
			level += 1;
			x = level;
			y = 0;
		}
		else {
			x -= 1;
			y += 1;
		}
		counter += 1;
	}
	return (x, y);
}

// ============================================================================================== //

#[derive(Component)]
pub struct MapPosition {
	pub x: f64,
	pub y: f64
}

// ============================================================================================== //
// Map Position To Map Quadrant, Chunk, & Index Values

pub fn map_to_data_position(x_map: i64, y_map: i64) -> (u64, u64, u64) {
	let quadrant: u64;
	let chunk_number: u64;
	let index: u64;

	// Get quadrant
	match (x_map >= 0, y_map >= 0) {
		(true, true) => {
			quadrant = 0;
		}
		(false, true) => {
			quadrant = 1;
		}
		(false, false) => {
			quadrant = 2;
		}
		(true, false) => {
			quadrant = 3;
		}
	}

	// Get chunk
	let x_abs: u64 = x_map.abs() as u64;
	let y_abs: u64 = y_map.abs() as u64;
	chunk_number = get_chunk_number(x_abs / (CHUNK_SIZE as u64), y_abs / (CHUNK_SIZE as u64));

	// Get index
	let x_chunk: u64 = x_abs % (CHUNK_SIZE as u64);
	let y_chunk: u64 = y_abs % (CHUNK_SIZE as u64);
	index = x_chunk + (CHUNK_SIZE as u64) * y_chunk;

	// Return
	return (quadrant, chunk_number, index);
}

pub fn data_to_map_position(quadrant: u64, chunk_number: usize, index: usize) -> (i64, i64) {
	let x_map: i64;
	let y_map: i64;

	let (x_main, y_main) = get_x_y_from_chunk(chunk_number);

	let x_remainder: u64 = (index as u64) % (CHUNK_SIZE as u64);
	let y_remainder: u64 = (index as u64) / (CHUNK_SIZE as u64);

	let x_sign: i64;	// 1 if x is positive, -1 otherwise
	let y_sign: i64;	// 1 if y is positive, -1 otherwise
	let x_offset: i64;
	let y_offset: i64;
	match quadrant {
		1 => {
			x_sign = -1;
			y_sign = 1;
			x_offset = (CHUNK_SIZE as i64) * (x_main as i64) + 1;
			y_offset = (CHUNK_SIZE as i64) * (y_main as i64) + 0;
		}
		2 => {
			x_sign = -1;
			y_sign = -1;
			x_offset = (CHUNK_SIZE as i64) * (x_main as i64) + 1;
			y_offset = (CHUNK_SIZE as i64) * (y_main as i64) + 1;
		}
		3 => {
			x_sign = 1;
			y_sign = -1;
			x_offset = (CHUNK_SIZE as i64) * (x_main as i64) + 0;
			y_offset = (CHUNK_SIZE as i64) * (y_main as i64) + 1;
		}
		_ => {
			x_sign = 1;
			y_sign = 1;
			x_offset = (CHUNK_SIZE as i64) * (x_main as i64) + 0;
			y_offset = (CHUNK_SIZE as i64) * (y_main as i64) + 0;
		}
	}

	x_map = ((x_main * (CHUNK_SIZE as u64) + x_remainder) as i64 + x_offset) * x_sign;
	y_map = ((y_main * (CHUNK_SIZE as u64) + y_remainder) as i64 + y_offset) * y_sign;

	return (x_map, y_map);
}

// ============================================================================================== //

#[derive(Resource)]
pub struct Map {
	quadrant_0: Vec<Chunk>,
	quadrant_1: Vec<Chunk>,
	quadrant_2: Vec<Chunk>,
	quadrant_3: Vec<Chunk>
}

impl Default for Map {
	fn default() -> Self {
		Map {
			quadrant_0: Vec::new(),
			quadrant_1: Vec::new(),
			quadrant_2: Vec::new(),
			quadrant_3: Vec::new()
		}
	}
}

impl Map {
	pub fn get_chunk(&mut self, x_map: i64, y_map: i64) -> &mut Chunk {
		let (quadrant, chunk_number, _index) = map_to_data_position(x_map, y_map);

		match quadrant {
			1 => {
				return &mut self.quadrant_1[chunk_number as usize];
			}
			2 => {
				return &mut self.quadrant_2[chunk_number as usize];
			}
			3 => {
				return &mut self.quadrant_3[chunk_number as usize];
			}
			_ => {
				return &mut self.quadrant_0[chunk_number as usize];
			}
		}
	}

	pub fn get_tile(&mut self, x_map: i64, y_map: i64) -> &mut Tile {
		let (x_offset, y_offset) = match (x_map >= 0, y_map >= 0) {
			(true, true) => {(0, 0)} // Quadrant 0
			(false, true) => {(-1, 0)} // Quadrant 1
			(false, false) => {(-1, -1)} // Quadrant 2
			(true, false) => {(0, -1)} // Quadrant 3
		};

		let x_within_chunk: u64 = ((x_map.abs() + x_offset) as u64) % (CHUNK_SIZE as u64);
		let y_within_chunk: u64 = ((y_map.abs() + y_offset) as u64) % (CHUNK_SIZE as u64);
		let chunk: &mut Chunk = self.get_chunk(x_map, y_map);
		return chunk.get_tile(x_within_chunk as usize, y_within_chunk as usize);
	}

	pub fn change_tile_terrain(&mut self, x_map: i64, y_map: i64, new_terrain: Terrain) {
		let tile: &mut Tile = &mut self.get_tile(x_map, y_map);
		tile.terrain = new_terrain;
	}
}

// ============================================================================================== //

pub fn spawn_map(
	mut map: ResMut<Map>,
	mut commands: Commands,
	tile_textures: Res<TileTextures>,
) {
	// Set Up Map
	let map_type = TilemapType::Isometric(IsoCoordSystem::Diamond);

	// Set Up Quadrants 0, 1, 2, & 3
	for quadrant in 0..4 {
		// Set Up Bevy's Chunk
		let chunk_size = TilemapSize{x: CHUNK_SIZE as u32, y: CHUNK_SIZE as u32};
		let mut tile_storage: TileStorage = TileStorage::empty(chunk_size);
		let grid_size = TilemapGridSize{x: TILE_WIDTH, y: TILE_HEIGHT};
		let tilemap_size = TilemapTileSize{x: TILE_WIDTH, y: TILE_HEIGHT};
		let tilemap_entity: Entity = commands.spawn_empty().id();
		let chunk_width = CHUNK_SIZE as f32 * TILE_WIDTH;
    	let chunk_height = CHUNK_SIZE as f32 * TILE_HEIGHT;

		// Move the transform_offset in relation to the quadrant
		let transform_offset: Vec3;
		match quadrant {
			0 => {
				transform_offset = Vec3::new((0.5) * chunk_width, 0.0, 0.0);
			}
			1 => {
				transform_offset = Vec3::new(0.0, -chunk_height / 2.0, 0.0);
			}
			2 => {
				transform_offset = Vec3::new((-0.5)* chunk_width, 0.0, 0.0);
			}
			3 => {
				transform_offset = Vec3::new(0.0, chunk_height / 2.0, 0.0);
			}
			_ => {
				transform_offset = Vec3::ZERO;
			}
		}

		// My Chunk Struct
		let mut chunk: Chunk = Chunk::new(0, quadrant);

		for x in 0..CHUNK_SIZE {
			for y in 0..CHUNK_SIZE {
				let texture_index = 0;
				let terrain_type = Terrain::Grass;

				// My Tile Struct
				let index: usize = (x + y * CHUNK_SIZE) as usize;
				let (x_map, y_map) = data_to_map_position(
					quadrant,
					0,
					index
				);
				let tile = Tile::new(
					x_map,
					y_map,
					quadrant,
					terrain_type
				);

			// Add Tile to Chunk
			chunk.tiles.push(tile.clone());

			// Bevy's TilePos
			let tile_pos = TilePos{
				x: x.try_into().unwrap(),
				y: y.try_into().unwrap()
			};

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
			size: chunk_size,
			tile_size: tilemap_size,
			texture: tile_textures.textures.clone(),
			storage: tile_storage,
			anchor: TilemapAnchor::Center,
			map_type,
			transform: Transform::from_translation(transform_offset),
			..Default::default()
		});

		commands.entity(tilemap_entity).insert(chunk.clone());

		// Add Chunk to Map
		match quadrant {
			0 => {map.quadrant_0.push(chunk)}
			1 => {map.quadrant_1.push(chunk)}
			2 => {map.quadrant_2.push(chunk)}
			3 => {map.quadrant_3.push(chunk)}
			_ => {}
		}
	}
}

// ============================================================================================== //

pub fn update_terrain_to_sand(
	mut map: ResMut<Map>,
	mut tile_query: Query<(&Tile, &mut TileTextureIndex)>
) {
	let min_coord = -1;
	let max_coord = 1;
	let sand_texture_index = 2;

	// Iterate over all entities that have both a Tile and a TileTextureIndex
	for (tile, mut texture_index) in tile_query.iter_mut() {
		if tile.x_map >= min_coord 
			&& tile.x_map <= max_coord 
			&& tile.y_map >= min_coord 
			&& tile.y_map <= max_coord
		{
			// Set the new texture index
			texture_index.0 = sand_texture_index;

			map.change_tile_terrain(tile.x_map, tile.y_map, Terrain::Sand);
		}
		else if tile.x_map == 0 && tile.y_map == -4 {
			texture_index.0 = 1;
			map.change_tile_terrain(tile.x_map, tile.y_map, Terrain::Water);
		}
	}
}