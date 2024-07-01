use bevy::{math::{vec2, Vec2}, prelude::Component};
use rand::Rng;

#[derive(Component, PartialEq)]
pub struct Blob {
    pub vx: f32,
    pub vy: f32,
    pub vector: Vec2,
    pub blob_type: BlobType,
}

#[derive(Component, PartialEq)]
pub enum BlobType {
    WHITE,
    RED
}

impl Blob {
    pub fn default() -> Self {
        let vx = rand::thread_rng().gen_range(-200.0..200.0);
        let vy = rand::thread_rng().gen_range(-200.0..200.0);
        let vector = vec2(rand::thread_rng().gen_range(-200.0..200.0), rand::thread_rng().gen_range(-200.0..200.0));
        let blob_type = BlobType::RED;

        Self {vx,vy,vector, blob_type}
    }
}