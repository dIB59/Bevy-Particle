use std::sync::Mutex;

use criterion::{criterion_group, criterion_main, Criterion};
use bevy::math::Vec2;
use rand::Rng;

// Import Quadtree and Quad from their module
use particlebevy::quad_tree::{Quad, Quadtree};
use rayon::prelude::*;

fn insert_particles_benchmark(c: &mut Criterion) {
    // Set up the benchmark for inserting particles into the quadtree
    //Create 10000 random particles
    let mut rand = rand::rng();
    let particles = (0..100_000)
        .map(|_| {
            let x = rand.random_range(-500.0..500.0);
            let y = rand.random_range(-500.0..500.0);
            Vec2::new(x, y)
        })
        .collect::<Vec<_>>();

    c.bench_function("insert 10000 particles", |b| {
        b.iter(|| {
            // Create a new quadtree
            let mut quadtree = Quadtree::new(0.5, 0.1);
            // Create a bounding box for the quadtree
            let mut quad = Quad::new_containing(&particles);
            // Clear the quadtree with the bounding box
            quadtree.clear(quad);

            // Insert particles into the quadtree
            particles.iter().for_each(|&particle| {
                quadtree.insert(particle, 1.0); // Only if thread-safe!
            });

        });
    });
}

fn resolve_particle_collision_benchmark(c: &mut Criterion) {
    let mut rng = rand::rng();
    let particles = (0..10_000)
        .map(|_| {
            let x = rng.random_range(-500.0..500.0);
            let y = rng.random_range(-500.0..500.0);
            Vec2::new(x, y)
        })
        .collect::<Vec<_>>();

    c.bench_function("resolve_particle_collision", |b| {
        b.iter(|| {
            let mut quadtree = Quadtree::new(0.5, 0.1);
            let quad = Quad::new_containing(&particles);
            quadtree.clear(quad);

            for &particle in &particles {
                quadtree.insert(particle, 1.0);
            }
            quadtree.propagate(); // If part of your logic
        });
    });
}

criterion_group!(benches, insert_particles_benchmark);
criterion_group!(benches2, resolve_particle_collision_benchmark);
criterion_main!(benches, benches2);


