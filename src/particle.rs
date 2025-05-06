use bevy::{
    prelude::*,
    window::{Monitor, PrimaryWindow},
};
use rand::prelude::*;

pub type ParticleBundle = (
    Particle,
    Position,
    Mesh2d,
    MeshMaterial2d<ColorMaterial>,
    Transform,
);

const SCREEN_WIDTH: f32 = 1900.0;
const SCREEN_HEIGHT: f32 = 1200.0;

#[derive(Component, Default)]
pub struct Particle {
    pub vel_x: f32,
    pub vel_y: f32,
    pub mass: u32,
    pub radius: u32,
}

#[derive(Component, Default)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

/// Returns a color based on mass, interpolating through blue → green → yellow → red.
fn get_color_based_on_mass(mass: u32) -> Color {
    let hue = (mass as f32 % 75.0) / 75.0; // Normalize to [0.0, 1.0]
    let saturation = 1.0;
    let value = 1.0;

    Color::hsv(hue * 360.0, saturation, value)
}

pub fn create_particle(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
) -> ParticleBundle {
    let window = windows.single().unwrap(); // Get window
    let width = window.width();
    let height = window.height();
    let mut rng = rand::rng();
    let x = rng.random_range(-width / 2.5..width / 2.5) as f32;
    let y = rng.random_range(-height / 2.5..height / 2.5) as f32;
    let mass = rng.random_range(1..=5) * 10;
    let radius = mass / 10;

    (
        Particle {
            vel_x: rng.random_range(-0.5..0.5),
            vel_y: rng.random_range(-0.5..0.5),
            mass,
            radius,
        },
        Position { x, y },
        Mesh2d(meshes.add(Circle::new(radius as f32))),
        MeshMaterial2d(materials.add(get_color_based_on_mass(mass))),
        Transform::from_xyz(x, y, 0.0),
    )
}

pub fn update_position(mut query: Query<(&mut Position, &mut Particle, &mut Transform)>) {
    for (mut position, mut particle, mut transform) in query.iter_mut() {
        position.x += particle.vel_x;
        position.y += particle.vel_y;

        // Invert the velocity if the particle is outside the screen bounds
        if position.x < -SCREEN_WIDTH / 2.0 {
            particle.vel_x *= -1.0;
        } else if position.x > SCREEN_WIDTH / 2.0 {
            particle.vel_x *= -1.0;
        }

        if position.y < -SCREEN_HEIGHT / 2.0 {
            particle.vel_y *= -1.0;
        } else if position.y > SCREEN_HEIGHT / 2.0 {
            particle.vel_y *= -1.0;
        }

        // Update the transform
        transform.translation.x = position.x;
        transform.translation.y = position.y;
    }
}

pub fn resolve_particle_collision(
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
