// Autopilot - Player

// ============================================================================================== //
// Imports

use bevy::prelude::*;

use super::map::map::*;

// ============================================================================================== //

pub fn spawn_camera(mut commands: Commands) {
	commands.spawn(Camera2d);
}

#[derive(Component)]
pub struct Player;

pub fn spawn_player(mut commands: Commands) {
	commands.spawn(
		Player
	);
}