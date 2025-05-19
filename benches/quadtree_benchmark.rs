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
    let particles = (0..10_000)
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
            let quad = Quad::new_containing(&particles);
            // Clear the quadtree with the bounding box
            quadtree.clear(quad);

            // Insert particles into the quadtree
            for particle in &particles {
                quadtree.insert(*particle, 1.0);
            }
        });
    });
}

criterion_group!(benches, insert_particles_benchmark);
criterion_main!(benches);


