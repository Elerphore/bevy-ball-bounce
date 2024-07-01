use bevy::prelude::{Camera2dBundle, Commands};

use super::data::{camera::MainCamera, mouse::Mouse};


pub fn setup(mut commands: Commands) {
    commands.spawn(Mouse::default());
    commands.spawn((Camera2dBundle::default(), MainCamera));
}