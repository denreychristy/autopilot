// Autopilot - Main

// ============================================================================================== //
// Imports

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
//use rand::Rng;

mod modules;
use modules::map::{
	chunk::*,
	map_position::*,
	tile::*
};
use modules::player::*;

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
	
	app.add_systems(PreStartup, load_tile_textures);

	app.add_systems(Startup, spawn_map);
	app.add_systems(Startup, spawn_camera);
	app.add_systems(Startup, spawn_player);

	// ================================================== //
	// Run

	app.run();
}