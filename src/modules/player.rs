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
	commands.spawn((
		Player,
		MapPosition{ x: 0.0, y: 0.0 }
	));
}

pub fn print_player_position(query: Query<&MapPosition, With<Player>>) {
	for position in &query {
		println!("Player is located at ({}, {}).", position.x, position.y);
	}
}