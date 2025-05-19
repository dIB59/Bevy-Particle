use bevy::prelude::*;

use crate::particle::{Particle, Position};

#[derive(Clone, Copy, Debug)]
pub struct Quad {
    pub center: Vec2,
    pub size: f32,
}

#[derive(Resource)]
pub struct Quadtree {
    pub t_sq: f32,
    pub e_sq: f32,
    pub nodes: Vec<Node>,
    pub parents: Vec<usize>,
}


impl Quad {
    pub fn new_containing(positions: &[Vec2]) -> Self {
        let mut min = Vec2::splat(f32::MAX);
        let mut max = Vec2::splat(f32::MIN);

        for &pos in positions {
            min = min.min(pos);
            max = max.max(pos);
        }

        let center = (min + max) * 0.5;
        let size = (max.x - min.x).max(max.y - min.y);

        Self { center, size }
    }

    pub fn find_quadrant(&self, pos: Vec2) -> usize {
        ((pos.y > self.center.y) as usize) << 1 | (pos.x > self.center.x) as usize
    }

    pub fn into_quadrant(mut self, quadrant: usize) -> Self {
        self.size *= 0.5;
        self.center.x += ((quadrant & 1) as f32 - 0.5) * self.size;
        self.center.y += ((quadrant >> 1) as f32 - 0.5) * self.size;
        self
    }

    pub fn subdivide(&self) -> [Quad; 4] {
        [0, 1, 2, 3].map(|i| self.into_quadrant(i))
    }
}

#[derive(Clone)]
pub struct Node {
    pub children: usize,
    pub next: usize,
    pub pos: Vec2,
    pub mass: f32,
    pub quad: Quad,
}

impl Node {
    pub fn new(next: usize, quad: Quad) -> Self {
        Self {
            children: 0,
            next,
            pos: Vec2::ZERO,
            mass: 0.0,
            quad,
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.children == 0
    }

    pub fn is_empty(&self) -> bool {
        self.mass == 0.0
    }
}

impl Quadtree {
    pub const ROOT: usize = 0;

    pub fn new(theta: f32, epsilon: f32) -> Self {
        Self {
            t_sq: theta * theta,
            e_sq: epsilon * epsilon,
            nodes: Vec::new(),
            parents: Vec::new(),
        }
    }

    pub fn clear(&mut self, quad: Quad) {
        self.nodes.clear();
        self.parents.clear();
        self.nodes.push(Node::new(0, quad));
    }

    fn subdivide(&mut self, node: usize) -> usize {
        self.parents.push(node);
        let children = self.nodes.len();
        self.nodes[node].children = children;

        let nexts = [children + 1, children + 2, children + 3, self.nodes[node].next];
        let quads = self.nodes[node].quad.subdivide();
        for i in 0..4 {
            self.nodes.push(Node::new(nexts[i], quads[i]));
        }

        children
    }

    pub fn insert(&mut self, pos: Vec2, mass: f32) {
        let mut node = Self::ROOT;

        while self.nodes[node].children != 0 {
            let q = self.nodes[node].quad.find_quadrant(pos);
            node = self.nodes[node].children + q;
        }

        if self.nodes[node].is_empty() {
            self.nodes[node].pos = pos;
            self.nodes[node].mass = mass;
            return;
        }

        let (p, m) = (self.nodes[node].pos, self.nodes[node].mass);
        if pos == p {
            self.nodes[node].mass += mass;
            return;
        }

        loop {
            let children = self.subdivide(node);
            let q1 = self.nodes[node].quad.find_quadrant(p);
            let q2 = self.nodes[node].quad.find_quadrant(pos);

            if q1 == q2 {
                node = children + q1;
            } else {
                let n1 = children + q1;
                let n2 = children + q2;
                self.nodes[n1].pos = p;
                self.nodes[n1].mass = m;
                self.nodes[n2].pos = pos;
                self.nodes[n2].mass = mass;
                return;
            }
        }
    }

    pub fn propagate(&mut self) {
        for &node in self.parents.iter().rev() {
            let i = self.nodes[node].children;

            let mut pos = Vec2::ZERO;
            let mut mass = 0.0;
            for j in 0..4 {
                pos += self.nodes[i + j].pos * self.nodes[i + j].mass;
                mass += self.nodes[i + j].mass;
            }

            self.nodes[node].pos = if mass > 0.0 { pos / mass } else { Vec2::ZERO };
            self.nodes[node].mass = mass;
        }
    }

    pub fn acc(&self, pos: Vec2) -> Vec2 {
        let mut acc = Vec2::ZERO;
        let mut node = Self::ROOT;

        loop {
            let n = &self.nodes[node];
            let d = n.pos - pos;
            let d_sq = d.length_squared();

            if n.is_leaf() || n.quad.size * n.quad.size < d_sq * self.t_sq {
                let denom = (d_sq + self.e_sq) * d_sq.sqrt();
                acc += d * (n.mass / denom).min(f32::MAX);

                if n.next == 0 {
                    break;
                }
                node = n.next;
            } else {
                node = n.children;
            }
        }

        acc
    }
}

pub fn handle_quadtree_gravity(
    mut quadtree: ResMut<super::quad_tree::Quadtree>,
    mut query: Query<(&Position, &mut Particle)>,
) {
    // Clear and rebuild the quadtree
    quadtree.clear(Quad::new_containing(&[]));
    let positions: Vec<_> = query.iter().map(|(pos, _)|pos.clone()).collect();

    let vec2_positions: Vec<Vec2> = positions.iter().map(|pos| Vec2::new(pos.x, pos.y)).collect();
    let root_quad = super::quad_tree::Quad::new_containing(&vec2_positions);
    quadtree.clear(root_quad);

    for (pos, mut particle) in query.iter_mut() {
        quadtree.insert(Vec2::new(pos.x, pos.y), particle.mass as f32);
    }

    quadtree.propagate();
    // Apply gravity
    for (pos, mut particle) in query.iter_mut() {
        let pos_vec = Vec2::new(pos.x, pos.y);
        let force = quadtree.acc(pos_vec);

        // Simple acceleration assuming 1 unit time step
        particle.vel_x += force.x / 100.0; // Scale down the force for better simulation
        particle.vel_y += force.y / 100.0; // Scale down the force for better simulation
    }
}