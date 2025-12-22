// Autopilot - Tile

// ============================================================================================== //
// Imports

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::super::settings::*;

// ============================================================================================== //
// Tile Textures

#[derive(Resource)]
pub struct TileTextures {
	pub textures: TilemapTexture,
}

pub fn load_tile_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
	let mut tile_texture_handles: Vec<Handle<Image>> = Vec::with_capacity(102);
	for i in 0..100 {
		let filename = format!("tiles/grass/grass_{}.png", i);
		tile_texture_handles.push(
			asset_server.load(filename)
		)
	}
	tile_texture_handles.push(asset_server.load("tiles/water_0.png"));
	tile_texture_handles.push(asset_server.load("tiles/sand_0.png"));

	let tilemap_textures: TilemapTexture = TilemapTexture::Vector(tile_texture_handles);

	commands.insert_resource(TileTextures {textures: tilemap_textures});
}

// ============================================================================================== //
// struct Tile

#[derive(Clone, Component, Debug)]
pub struct Tile {
	pub x_map: i64,			// Where this Tile exists on the Map - x
	pub y_map: i64,			// Where this Tile exists on the Map - y
	pub bevy_id: Entity		// Bevy's component entity id
}

// ============================================================================================== //
// impl Tile

impl Tile {
	pub fn new(bevy_id: Entity, x_map: i64, y_map: i64) -> Tile {
		Tile {bevy_id, x_map, y_map}
	}
}