// AutoPilot - Modules - World - Tile

// ============================================================================================== //

use bevy::prelude::*;

// ============================================================================================== //

enum Grass {
	Variation_1
}

// ============================================================================================== //

#[derive(Component)]
pub struct Tile {
	pub content: Option<Grass>,
	pub discovered: bool,
	pub visible: bool
}

impl Tile {
	pub fn new() -> Self {
		Tile {
			content: None,
			discovered: false,
			visible: false
		}
	}
}