// Autopilot - Main

// ============================================================================================== //
// Imports

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
//use rand::Rng;

// ============================================================================================== //
// Constants

const CHUNK_SIZE: usize	= 20;
const TILE_WIDTH: f32	= 64.0;
const TILE_HEIGHT: f32	= 32.0;

// ============================================================================================== //
// Main

fn main() {
	let mut app = App::new();

	// ================================================== //
	// Plugins
	app.add_plugins(
		DefaultPlugins
			.set(WindowPlugin{
				primary_window: Some(
					Window {
						title: String::from(
							"Autopilot"
						),
						..Default::default()
					}
				),
				..default()
			})
			.set(ImagePlugin::default_nearest()
			)
	);
	app.add_plugins(TilemapPlugin);

	// ================================================== //
	// Systems
	app.add_systems(Startup, spawn_map);
	app.add_systems(Startup, spawn_camera);
	app.add_systems(Startup, spawn_player);

	// ================================================== //
	// Run
	app.run();
}

// ============================================================================================== //
// Components

#[derive(Component)]
pub struct WorldPosition {
	pub x: f64,
	pub y: f64
}

// ============================================================================================== //
// Map Elements

#[derive(Clone)]
pub enum Terrain {
	Grass,
	Water,
	Sand
}

#[derive(Clone, Component)]
pub struct Tile {
	pub x_map: i64,			// Where this Tile exists on the Map - x
	pub y_map: i64,			// Where this Tile exists on the Map - y
	pub terrain: Terrain	// Terrain type, like grass, desert, ocean, etc.
}

#[derive(Component)]
pub struct Chunk {
	pub tiles: Vec<Tile>	// List of tiles, in a 100x100 grid
}

impl Chunk {
	pub fn new() -> Self {
		let tiles: Vec<Tile> = Vec::with_capacity(CHUNK_SIZE * CHUNK_SIZE);
		Chunk {
			tiles
		}
	}

	/// This function returns a clone of the Tile object located at the (x_chunk, y_chunk)
	/// coordinate pair *within the chunk itself*, as opposed to the global map (x, y) coordinate
	/// pair.
	pub fn get_tile(&self, x_chunk: usize, y_chunk: usize) -> Tile {
		let index: usize = CHUNK_SIZE * y_chunk + x_chunk;
		return self.tiles[index].clone();
	}

	/// This function changes the Terrain type of an existing Tile object.
	pub fn update_tile_terrain(&mut self, x_chunk: usize, y_chunk: usize, new_terrain: Terrain) {
		let index: usize = CHUNK_SIZE * y_chunk + x_chunk;
		self.tiles[index].terrain = new_terrain;
	}
}

pub fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>) {
	spawn_chunk(commands, asset_server, 0, 0);
}

pub fn spawn_chunk(mut commands: Commands, asset_server: Res<AssetServer>, x_map_offset: i64, y_map_offset: i64) {
	// Load Tile Images
	let texture_vec = TilemapTexture::Vector(vec![
		asset_server.load("tiles/grass_0.png"),
		asset_server.load("tiles/water_0.png"),
		asset_server.load("tiles/sand_0.png")
	]);

//	const WATER_PERCENT: f32 = 20.0;
//	const SAND_PERCENT: f32 = 10.0;
//	const GRASS_START: f32 = WATER_PERCENT + SAND_PERCENT;

//	let mut rng = rand::rng();

	// Setup map dimensions
	let map_size = TilemapSize{x: CHUNK_SIZE as u32, y: CHUNK_SIZE as u32};
	let grid_size = TilemapGridSize{x: TILE_WIDTH, y: TILE_HEIGHT};
	let tilemap_size = TilemapTileSize{x: TILE_WIDTH, y: TILE_HEIGHT};
	let map_type = TilemapType::Isometric(IsoCoordSystem::Diamond);

	let mut tile_storage = TileStorage::empty(map_size);
	let tilemap_entity = commands.spawn_empty().id();

	// Create a new Chunk
	let mut chunk = Chunk::new();

	for x in 0..CHUNK_SIZE {
		for y in 0..CHUNK_SIZE {
//			let random_value: f32 = rng.random_range(0.0 .. 100.0);
//
//			let (texture_index, terrain_type) = match random_value {
//				0.0 .. WATER_PERCENT => (1, Terrain::Water), 
//				WATER_PERCENT .. GRASS_START => (2, Terrain::Sand),
//				_ => (0, Terrain::Grass),
//			};

			let texture_index = 0;
			let terrain_type = Terrain::Grass;

			let tile = Tile {
				x_map: x as i64,
				y_map: y as i64,
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
		texture: texture_vec,
		storage: tile_storage,
		anchor: TilemapAnchor::Center,
		map_type,
		..Default::default()
	});

	commands.entity(tilemap_entity).insert(chunk);
}

// ============================================================================================== //
// Player

pub fn spawn_camera(mut commands: Commands) {
	commands.spawn(Camera2d);
}

#[derive(Component)]
pub struct Player;

pub fn spawn_player(mut commands: Commands) {
	commands.spawn((
		Player,
		WorldPosition{ x: 0.0, y: 0.0 }
	));
}

pub fn print_player_position(query: Query<&WorldPosition, With<Player>>) {
	for position in &query {
		println!("Player is located at ({}, {}).", position.x, position.y);
	}
}