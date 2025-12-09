// Autopilot - Player

// ============================================================================================== //
// Imports

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

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