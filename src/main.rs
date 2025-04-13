use bevy::app::Update;
use bevy::asset::Assets;
use bevy::core_pipeline::core_2d::Camera2d;
use bevy::ecs::system::Query;
use bevy::ecs::system::ResMut;
use bevy::prelude::*;
use bevy::render::mesh::Mesh2d;
use bevy::sprite::ColorMaterial;
use bevy::transform::components::Transform;
use bevy::{DefaultPlugins, app::Startup, prelude::App};

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update_position)
        .run();
}

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

#[derive(Component, Default)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Default)]
struct Particle {
    vel_x: f32,
    vel_y: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera
    commands.spawn(Camera2d::default());

    // Create a circular mesh and material
    let circle = meshes.add(Circle::new(5.0)); // radius 5.0
    let material = materials.add(Color::WHITE);

    // Spawn the particle with a circular mesh
    commands.spawn((
        Particle {
            vel_x: 2.0,
            vel_y: 1.5,
        },
        Position { x: 0.0, y: 0.0 },
        Mesh2d(circle),
        MeshMaterial2d(material),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn update_position(mut query: Query<(&mut Position, &mut Particle, &mut Transform)>) {
    for (mut position, mut particle, mut transform) in query.iter_mut() {
        position.x += particle.vel_x;
        position.y += particle.vel_y;

        if position.x <= -SCREEN_WIDTH / 2.0 || position.x >= SCREEN_WIDTH / 2.0 {
            particle.vel_x = -particle.vel_x;
        }

        if position.y <= -SCREEN_HEIGHT / 2.0 || position.y >= SCREEN_HEIGHT / 2.0 {
            particle.vel_y = -particle.vel_y;
        }

        transform.translation.x = position.x;
        transform.translation.y = position.y;
    }
}
