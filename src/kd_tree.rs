use crate::particle;
use crate::particle::Position;
use crate::particle::Particle;
use bevy::prelude::*;

pub fn handle_collision_kd_tree(
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
        let (pos_i_clone, par_i) = {
            let (pos, par) = &particles[i];
            ([pos.x, pos.y], par)
        };

        search_radius(&tree, pos_i_clone, (par_i.radius * 2) as f32, 0, &mut neighbors);

        for &j in neighbors.iter() {
            if i == j {
                continue;
            }

            let (i, j) = if i < j { (i, j) } else { (j, i) };
            let (left, right) = particles.split_at_mut(j);
            let (pi, pj) = (&mut left[i], &mut right[0]);
            let total_radius = pi.1.radius + pj.1.radius;
            particle::resolve_particle_collision(&mut pi.0, &mut pi.1, &mut pj.0, &mut pj.1, total_radius as f32);
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

