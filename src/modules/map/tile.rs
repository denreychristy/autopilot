// Autopilot - Tile

// ============================================================================================== //
// Imports

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::super::settings::*;

// ============================================================================================== //
// Constants

// ============================================================================================== //

#[derive(Resource)]
pub struct TileTextures {
	pub textures: TilemapTexture,
}

pub fn load_tile_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
	let tile_texture_handles: Vec<Handle<Image>> = vec![
		asset_server.load("tiles/grass_0.png"),
		asset_server.load("tiles/water_0.png"),
		asset_server.load("tiles/sand_0.png"),
	];

	let tilemap_textures: TilemapTexture = TilemapTexture::Vector(tile_texture_handles);

	commands.insert_resource(TileTextures {textures: tilemap_textures});
}

#[derive(Clone, Debug)]
pub enum Terrain {
	Grass,
	Water,
	Sand
}

#[derive(Clone, Component, Debug)]
pub struct Tile {
	pub x_map: i64,			// Where this Tile exists on the Map - x
	pub y_map: i64,			// Where this Tile exists on the Map - y
	pub quadrant: u64,		// Where this Tile exists on the Map - Quadrant
	pub terrain: Terrain	// Terrain type, like grass, desert, ocean, etc.
}

impl Tile {
	pub fn new(x_map: i64, y_map: i64, quadrant: u64, terrain: Terrain) -> Tile {
		Tile {x_map, y_map, quadrant, terrain}
	}
}