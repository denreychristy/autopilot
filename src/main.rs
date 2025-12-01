// Autopilot - Main

use bevy::prelude::*;

// ============================================================================================== //
// Notes
//
// App.add_startup_system(function name) | add a function that only runs at startup
// App.add_system(function name) | add a function to the game loop
// #[derive(Component)] | decorator for struct components
//

// Custom Plugins
//	pub struct SomePlugin;
//
//	impl Plugin for SomePlugin {
//		fn build(&self, app: &mut App) {
//			// add things to your app here
//		}
//	}

// ============================================================================================== //
// Main

fn main() {
    App::new()
		.add_plugins(DefaultPlugins)
		.add_systems(Startup, setup)
		.run();
}

// ============================================================================================== //

pub fn setup(mut commands: Commands) {
	commands.spawn(Player{ health: 100 });
}

// ============================================================================================== //

#[derive(Component)]
pub struct Player {
	pub health: u64
}

pub fn print_player_health(player_query: Query<&Player>) {
	for player in player_query {
		println!("Player health: {}", player.health)
	}
}