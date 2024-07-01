use bevy::{
    app::{App, FixedUpdate, Plugin},
    math::{
        bounding::{BoundingCircle, IntersectsVolume},
        Vec2,
    },
    prelude::{IntoSystemConfigs, Query, Res},
    time::Time,
    transform::components::Transform,
};

use crate::boids::data::blob::{Blob, BlobType};

const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);

pub struct BlobPlugin;

impl Plugin for BlobPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (bounds_blobs, interseption_blobs, move_blobs).chain(),
        );
    }
}

fn move_blobs(mut q: Query<(&Blob, &mut Transform)>, time: Res<Time>) {
    for (blob, mut transform) in q.iter_mut() {
        transform.translation.x += blob.vx * time.delta_seconds();
        transform.translation.y += blob.vy * time.delta_seconds();
    }
}

fn bounds_blobs(mut q: Query<(&mut Blob, &Transform)>) {
    for (mut blob, transform) in q.iter_mut() {
        if transform.translation.x < -BOUNDS.x || transform.translation.x > BOUNDS.x {
            blob.vx = -blob.vx;
        }

        if transform.translation.y < -BOUNDS.y || transform.translation.y > BOUNDS.y {
            blob.vy = -blob.vy;
        }
    }
}

fn interseption_blobs(mut q: Query<(&mut Blob, &mut Transform)>) {
    let mut combinations = q.iter_combinations_mut();

    while let Some([mut blob_one, mut blob_two]) = combinations.fetch_next() {
        let b_circle = BoundingCircle::new(blob_one.1.translation.truncate(), 60.0 / 2.0);
        let inner_b_circle = BoundingCircle::new(blob_two.1.translation.truncate(), 60.0 / 2.0);

        if !b_circle.intersects(&inner_b_circle) {
            continue;
        }

        let center = b_circle.center - inner_b_circle.center;
        let speed_vx = blob_one.0.vx - blob_two.0.vx;
        let speed_vy = blob_one.0.vy - blob_two.0.vy;

        let centers = (center.x * speed_vx) + (center.y * speed_vy) < 0.0;

        if blob_one.0.blob_type.eq(&BlobType::RED) && blob_two.0.blob_type.eq(&BlobType::RED) && centers {
            blob_one.0.vx = -blob_one.0.vx;
            blob_one.0.vy = -blob_one.0.vy;

            blob_two.0.vx = -blob_two.0.vx;
            blob_two.0.vy = -blob_two.0.vy;
        }

        if blob_one.0.blob_type.eq(&BlobType::RED) && blob_two.0.blob_type.eq(&BlobType::WHITE) && centers {
            blob_one.0.vx = -blob_one.0.vx;
            blob_one.0.vy = -blob_one.0.vy;
        }

        if blob_one.0.blob_type.eq(&BlobType::WHITE) && blob_two.0.blob_type.eq(&BlobType::RED) && centers {
            blob_two.0.vx = -blob_two.0.vx;
            blob_two.0.vy = -blob_two.0.vy;
        }
    }
}
