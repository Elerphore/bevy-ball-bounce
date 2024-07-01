mod boids;

use bevy::prelude::*;
use boids::{app::setup, plugins::{blob_plugin::BlobPlugin, mouse_plugin::MousePlugin}};


fn main() {
    App::new()
    .add_plugins((DefaultPlugins, MousePlugin, BlobPlugin))
    .insert_resource(Time::<Fixed>::from_hz(60.0))
    .add_systems(Startup, setup)
    .add_systems(Update, bevy::window::close_on_esc)
    .run();
}



