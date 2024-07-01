use bevy::{math::Vec2, prelude::Component};

#[derive(Component, Debug)]
pub struct Mouse {
    pub pos: Vec2
}

impl Mouse {
    pub fn default() -> Self {
        let pos = Vec2 {x: 0.0, y:0.0};
        Self {
            pos: pos,
        }
    }
}