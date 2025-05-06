use bevy::app::Update;
use bevy::asset::Assets;
use bevy::core_pipeline::core_2d::Camera2d;
use bevy::ecs::system::Query;
use bevy::ecs::system::ResMut;
use bevy::prelude::*;
use bevy::render::mesh::Mesh2d;
use bevy::sprite::ColorMaterial;
use bevy::transform::components::Transform;
use bevy::window::PresentMode;
use bevy::window::WindowMode;
use bevy::{DefaultPlugins, app::Startup, prelude::App};
use bevy_dev_tools::fps_overlay::FpsOverlayPlugin;
use particle::Position;
use particle::Particle;
use rand::prelude::*;

mod particle;
mod kd_tree;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Particle Simulation".to_string(),
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                present_mode: PresentMode::default(),
                ..default()
            }),
            ..default()
        }), FpsOverlayPlugin::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, (kd_tree::handle_collision_kd_tree, particle::update_position).chain())
        .run();
}

const SCREEN_WIDTH: f32 = 1900.0;
const SCREEN_HEIGHT: f32 = 1200.0;
const NUMBER_PARTICLES: u32 = 10000;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
) {
    // Camera
    commands.spawn(Camera2d::default());

    // Spawn the particle with a circular mesh
    for _ in 0..NUMBER_PARTICLES {
        commands.spawn(particle::create_particle(&mut meshes, &mut materials, windows));
    }
}
