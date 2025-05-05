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
use bevy::{DefaultPlugins, app::Startup, prelude::App};
use bevy_dev_tools::fps_overlay::FpsOverlayPlugin;
use bevy_dev_tools::DevToolsPlugin;
use rand::prelude::*;


fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::Immediate, // Disable VSync
                ..default()
            }),
            ..default()
        }), DevToolsPlugin, FpsOverlayPlugin::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, (update_position, handle_collision_kd_tree).chain())
        .run();
}

const COLLISION_RADIUS: f32 = 10.0;
const SCREEN_WIDTH: f32 = 1000.0;
const SCREEN_HEIGHT: f32 = 1000.0;
const NUMBER_PARTICLES: u32 = 5000;

#[derive(Component, Default)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Default)]
struct Particle {
    vel_x: f32,
    vel_y: f32,
    mass: u32,
    radius: u32
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera
    commands.spawn(Camera2d::default());

    // Create a circular mesh and material
    let material = materials.add(Color::WHITE);

    // Spawn the particle with a circular mesh
    for _ in 0..NUMBER_PARTICLES {
        commands.spawn(create_particle(&mut meshes, material.clone()));
    }
}

type ParticleBundle = (Particle, Position, Mesh2d, MeshMaterial2d<ColorMaterial>, Transform);

fn create_particle(meshes: &mut ResMut<Assets<Mesh>>, material: Handle<ColorMaterial>) -> ParticleBundle {
    let mut rng = rand::rng();
    let x = rng.random_range(-SCREEN_WIDTH/2.5..SCREEN_WIDTH/2.5) as f32;
    let y = rng.random_range(-SCREEN_HEIGHT/2.5..SCREEN_HEIGHT/2.5) as f32;
    let mass= rng.random_range(1..5);
    
    (
        Particle {
            vel_x: rng.random_range(-0.5..0.5),
            vel_y: rng.random_range(-0.5..0.5),
            mass,
            radius: mass
        
        },
        Position { x, y },
        Mesh2d(meshes.add(Circle::new(mass as f32))),
        MeshMaterial2d(material),
        Transform::from_xyz(x, y, 0.0),
    )
}

fn update_position(mut query: Query<(&mut Position, &mut Particle, &mut Transform)>) {
    for (mut position, particle, mut transform) in query.iter_mut() {
        position.x += particle.vel_x;
        position.y += particle.vel_y;

        // Wrap around the screen boundaries
        if position.x < -SCREEN_WIDTH / 2.0 {
            position.x = SCREEN_WIDTH / 2.0; // wrap to the right
        } else if position.x > SCREEN_WIDTH / 2.0 {
            position.x = -SCREEN_WIDTH / 2.0; // wrap to the left
        }

        if position.y < -SCREEN_HEIGHT / 2.0 {
            position.y = SCREEN_HEIGHT / 2.0; // wrap to the top
        } else if position.y > SCREEN_HEIGHT / 2.0 {
            position.y = -SCREEN_HEIGHT / 2.0; // wrap to the bottom
        }

        // Update the transform
        transform.translation.x = position.x;
        transform.translation.y = position.y;
    }
}


fn handle_collision_kd_tree(
    mut query: Query<(&mut Position, &mut Particle)>
) {
    let mut points: Vec<(usize, [f32; 2])> = query
        .iter()
        .enumerate()
        .map(|(i, (pos, _))| (i, [pos.x, pos.y]))
        .collect();

    let tree = build_kd_tree(&mut points, 0);
    let mut particles: Vec<_> = query.iter_mut().collect();

    for i in 0..particles.len() {
        let mut neighbors = Vec::new();
        let pos_i_clone = {
            let (pos, _) = &particles[i];
            [pos.x, pos.y]
        };

        

        search_radius(&tree, pos_i_clone, COLLISION_RADIUS, 0, &mut neighbors);

        for &j in neighbors.iter() {
            if i == j {
                continue;
            }

            let (i, j) = if i < j { (i, j) } else { (j, i) };
            let (left, right) = particles.split_at_mut(j);
            let (pi, pj) = (&mut left[i], &mut right[0]);
            let total_radius = pi.1.radius + pj.1.radius;
            resolve_particle_collision(&mut pi.0, &mut pi.1, &mut pj.0, &mut pj.1, total_radius as f32);
        }
    }
}

fn resolve_particle_collision(
    pos_i: &mut Position,
    particle_i: &mut Particle,
    pos_j: &mut Position,
    particle_j: &mut Particle,
    radius: f32,
) {
    let dx = pos_i.x - pos_j.x;
    let dy = pos_i.y - pos_j.y;
    let dist_sq = dx * dx + dy * dy;

    if dist_sq < radius * radius && dist_sq > 0.0 {
        let dist = dist_sq.sqrt();
        let nx = dx / dist;
        let ny = dy / dist;

        let mass_i = particle_i.mass as f32;
        let mass_j = particle_j.mass as f32;
        let total_mass = mass_i + mass_j;

        // Calculate overlap
        let overlap = radius - dist;

        // Push particles apart based on mass ratio
        pos_i.x += nx * overlap * (mass_j / total_mass);
        pos_i.y += ny * overlap * (mass_j / total_mass);
        pos_j.x -= nx * overlap * (mass_i / total_mass);
        pos_j.y -= ny * overlap * (mass_i / total_mass);

        // Relative velocity
        let dvx = particle_i.vel_x - particle_j.vel_x;
        let dvy = particle_i.vel_y - particle_j.vel_y;

        // Velocity along the collision normal
        let vn = dvx * nx + dvy * ny;

        if vn < 0.0 {
            // Elastic collision impulse with mass consideration
            let impulse = (2.0 * vn) / total_mass;
            particle_i.vel_x -= impulse * mass_j * nx;
            particle_i.vel_y -= impulse * mass_j * ny;
            particle_j.vel_x += impulse * mass_i * nx;
            particle_j.vel_y += impulse * mass_i * ny;
        }
    }
}


#[derive(Debug)]
struct KdNode {
    point: [f32; 2],
    index: usize, // index into your particle list
    left: Option<Box<KdNode>>,
    right: Option<Box<KdNode>>,
}

fn build_kd_tree(points: &mut [(usize, [f32; 2])], depth: usize) -> Option<Box<KdNode>> {
    if points.is_empty() {
        return None;
    }

    let axis = depth % 2;
    points.sort_by(|a, b| a.1[axis].partial_cmp(&b.1[axis]).unwrap());

    let mid = points.len() / 2;
    let (index, point) = points[mid];

    Some(Box::new(KdNode {
        point,
        index,
        left: build_kd_tree(&mut points[..mid], depth + 1),
        right: build_kd_tree(&mut points[mid + 1..], depth + 1),
    }))
}

fn search_radius(
    node: &Option<Box<KdNode>>,
    target: [f32; 2],
    radius: f32,
    depth: usize,
    results: &mut Vec<usize>,
) {
    if let Some(n) = node {
        let axis = depth % 2;
        let dist_sq = (n.point[0] - target[0]).powi(2) + (n.point[1] - target[1]).powi(2);

        if dist_sq <= radius.powi(2) {
            results.push(n.index);
        }

        let diff = target[axis] - n.point[axis];

        if diff <= radius {
            search_radius(&n.left, target, radius, depth + 1, results);
        }

        if diff >= -radius {
            search_radius(&n.right, target, radius, depth + 1, results);
        }
    }
}

