use bevy::app::Update;
use bevy::asset::Assets;
use bevy::core_pipeline::core_2d::Camera2d;
use bevy::ecs::system::Query;
use bevy::ecs::system::ResMut;
use bevy::prelude::*;
use bevy::sprite::ColorMaterial;
use bevy::window::PresentMode;
use bevy::window::WindowMode;
use bevy::{DefaultPlugins, app::Startup, prelude::App};
use bevy_dev_tools::fps_overlay::FpsOverlayPlugin;

mod kd_tree;
mod particle;
mod quad_tree;
mod simulation;
mod controls;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Particle Simulation".to_string(),
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                    present_mode: PresentMode::default(),
                    ..default()
                }),
                ..default()
            }),
            FpsOverlayPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (controls::camera_pan_keyboard, controls::camera_zoom).chain())
        .add_systems(
            Update,
            (kd_tree::handle_collision_kd_tree, quad_tree::handle_quadtree_gravity, particle::update_position).chain(),
        )
        .run();
}



fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
) {
    // Camera
    commands.spawn(Camera2d::default());
    commands.insert_resource(quad_tree::Quadtree::new(0.50, 0.3)); // Adjust theta/epsilon as needed

    // Spawn the particle with a circular mesh
    for _ in 0..simulation::NUMBER_PARTICLES {
        commands.spawn(particle::create_particle(
            &mut meshes,
            &mut materials
        ));
    }
}
