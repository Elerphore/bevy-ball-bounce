use bevy::{
    input::{mouse::MouseButtonInput, ButtonState}, math::vec2, prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}, window::PrimaryWindow
};

use crate::boids::data::{blob::{Blob, BlobType}, camera::MainCamera, mouse::Mouse};

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (mouse_move, mouse_click).chain());
    }
}

fn mouse_move(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut mouse_query: Query<&mut Mouse>,
) {
    let (camera, camera_transform) = q_camera.single();

    let mut mouse = mouse_query
        .get_single_mut()
        .expect("Error: No mouse object found");

    if let Some(poisition) = q_windows
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mouse.pos = poisition
    }
}

fn mouse_click(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mouse_query: Query<&Mouse>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mouse = mouse_query
        .get_single()
        .expect("Error: No mouse object found");

    for event in mouse_button_input_events.read() {
        if event.state == ButtonState::Pressed && event.button == MouseButton::Left {
            commands.spawn(spawn_blob(mouse, &mut materials, &mut meshes));
        }

        if event.state == ButtonState::Pressed && event.button == MouseButton::Right {
            commands.spawn(spawn_block(mouse, &mut materials, &mut meshes));
        }
    }
}

fn spawn_blob(
    mouse: &Mouse,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
) -> (Blob, MaterialMesh2dBundle<ColorMaterial>) {
    return (
        Blob::default(),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
            material: materials.add(Color::RED),
            transform: Transform::from_xyz(mouse.pos.x, mouse.pos.y, 0.0),
            ..default()
        },
    );
}

fn spawn_block(
    mouse: &Mouse,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
) -> (Blob, MaterialMesh2dBundle<ColorMaterial>) {
    return (
        Blob {
            vx: 0.0,
            vy: 0.0,
            vector: vec2(0.0, 0.0),
            blob_type: BlobType::WHITE,
        },
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(mouse.pos.x, mouse.pos.y, 0.0),
            ..default()
        },
    );
}