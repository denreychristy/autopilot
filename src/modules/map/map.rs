// Autopilot - Map

// ============================================================================================== //
// Imports

use num::integer::div_floor;
use rand::Rng;

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
	pub x_map: i64,
	pub y_map: i64
}

// ============================================================================================== //
// Map Position To Map Quadrant, Chunk, & Index Values

pub fn map_to_data_position(x_map: i64, y_map: i64) -> (u64, u64, u64) {
	let quadrant: u64;
	let chunk_number: u64;
	let index: u64;

	let x_offset: i64;
	let y_offset: i64;

	match (x_map >= 0, y_map >= 0) {
		(true, true) => {
			quadrant = 0;
			x_offset = 0;
			y_offset = 0;
		}
		(false, true) => {
			quadrant = 1;
			x_offset = 1;
			y_offset = 0;
		}
		(false, false) => {
			quadrant = 2;
			x_offset = 1;
			y_offset = 1;
		}
		(true, false) => {
			quadrant = 3;
			x_offset = 0;
			y_offset = 1;
		}
	}

	let x_abs: u64 = (x_map + x_offset).abs() as u64;
	let y_abs: u64 = (y_map + y_offset).abs() as u64;
	let x_chunk: u64 = div_floor(x_abs, CHUNK_SIZE as u64);
	let y_chunk: u64 = div_floor(y_abs, CHUNK_SIZE as u64);
	let x_within_chunk: u64 = x_abs / (CHUNK_SIZE as u64);
	let y_within_chunk: u64 = y_abs % (CHUNK_SIZE as u64);

	chunk_number = get_chunk_number(x_chunk, y_chunk);
	index = x_within_chunk + y_within_chunk * (CHUNK_SIZE as u64);

	return (quadrant, chunk_number, index);
}

pub fn data_to_map_position(quadrant: u64, chunk_number: usize, index: usize) -> (i64, i64) {
	let x_map: i64;
	let y_map: i64;

	let (x_chunk, y_chunk) = get_x_y_from_chunk(chunk_number);
	let x_within_chunk: u64 = (index as u64) / (CHUNK_SIZE as u64);
	let y_within_chunk: u64 = (index as u64) % (CHUNK_SIZE as u64);

	let x_sign: i64;
	let y_sign: i64;
	let x_offset: i64;
	let y_offset: i64;
	match quadrant {
		1 => {
			x_sign = -1;
			y_sign = 1;
			x_offset = -1;
			y_offset = 0;
		}
		2 => {
			x_sign = -1;
			y_sign = -1;
			x_offset = -1;
			y_offset = -1;
		}
		3 => {
			x_sign = 1;
			y_sign = -1;
			x_offset = 0;
			y_offset = -1;
		}
		_ => {
			x_sign = 1;
			y_sign = 1;
			x_offset = 0;
			y_offset = 0;
		}
	}

	x_map = x_sign * ((x_chunk * (CHUNK_SIZE as u64) + x_within_chunk) as i64) + x_offset;
	y_map = y_sign * ((y_chunk * (CHUNK_SIZE as u64) + y_within_chunk) as i64) + y_offset;

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
		let (quadrant, chunk_number, index) = map_to_data_position(x_map, y_map);
		let chunk: &mut Chunk;

		match quadrant {
			1 => {
				chunk = &mut self.quadrant_1[chunk_number as usize];
			}
			2 => {
				chunk = &mut self.quadrant_2[chunk_number as usize];
			}
			3 => {
				chunk = &mut self.quadrant_3[chunk_number as usize];
			}
			_ => {
				chunk = &mut self.quadrant_0[chunk_number as usize];
			}
		}

		return &mut chunk.tiles[index as usize];
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

		let mut rng = rand::rng();

		// Move the transform_offset in relation to the quadrant
		let transform_offset: Vec3;
		match quadrant {
			1 => {
				transform_offset = Vec3::new(0.0, -chunk_height / 2.0, 0.0);
			}
			2 => {
				transform_offset = Vec3::new((-0.5) * chunk_width, 0.0, 0.0);
			}
			3 => {
				transform_offset = Vec3::new(0.0, chunk_height / 2.0, 0.0);
			}
			_ => {
				transform_offset = Vec3::new((0.5) * chunk_width, 0.0, 0.0);
			}
		}

		// My Chunk Struct
		let mut chunk: Chunk = Chunk::new(0, quadrant);

		for y in 0..CHUNK_SIZE {
			for x in 0..CHUNK_SIZE {
				let texture_index = rng.random_range(0..100);

				let index: usize = (x + y * CHUNK_SIZE) as usize;
				let (x_map, y_map) = data_to_map_position(
					quadrant,
					0,
					index
				);

				// Bevy's TilePos
				let bevy_x: u32;
				let bevy_y: u32;
				match quadrant {
					1 => {
						bevy_x = x as u32;
						bevy_y = (CHUNK_SIZE - 1 - y) as u32;
					}
					2 => {
						bevy_x = (CHUNK_SIZE - 1 - x) as u32;
						bevy_y = (CHUNK_SIZE - 1 - y) as u32;
					}
					3 => {
						bevy_x = (CHUNK_SIZE - 1 - x) as u32;
						bevy_y = y as u32;
					}
					_ => {
						bevy_x = x as u32;
						bevy_y = y as u32;
					}
				}
				let tile_pos = TilePos{
					x: bevy_x,
					y: bevy_y
				};

				let tile_entity = commands.spawn((
					TileBundle {
						position: tile_pos,
						tilemap_id: TilemapId(tilemap_entity),
						texture_index: TileTextureIndex(texture_index),
						color: TileColor(Color::WHITE),
						..Default::default()
					},
					MapPosition {
						x_map,
						y_map
					}
				)).id();

				tile_storage.set(&tile_pos, tile_entity);

				// My Tile Struct
				let tile = Tile::new(
					tile_entity,
					x_map,
					y_map,
				);

				// Add Tile to Chunk
				chunk.tiles.push(tile.clone());
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

pub fn change_terrain(
	map: &mut Map,
	tile_query: &mut Query<(&MapPosition, &mut TileTextureIndex)>,
	new_texture_index: u32,
	x_map: i64,
	y_map: i64
) {
	let tile = map.get_tile(x_map, y_map);
	let result = tile_query.get_mut(tile.bevy_id);
	match result {
		Ok((_bevy_tile, mut texture_index)) => {
			texture_index.0 = new_texture_index;
		}
		Err(e) => {
			eprintln!("Change terrain function failed: {}", e)
		}
	}
}

// ============================================================================================== //

pub fn update_terrain_to_sand(
	mut map: ResMut<Map>,
	mut tile_query: Query<(&MapPosition, &mut TileTextureIndex)>
) {
	let coordinates: Vec<Vec<i64>> = vec![vec![-2, -2], vec![-1, -1]];
	for coordinate in coordinates {
		change_terrain(
			&mut map,
			&mut tile_query,
			101,
			coordinate[0],
			coordinate[1]
		);
	}
}