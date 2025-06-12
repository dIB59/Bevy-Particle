pub(crate) const SCREEN_WIDTH: f32 = 8000.0;
pub(crate) const SCREEN_HEIGHT: f32 = 8000.0;
pub(crate) const NUMBER_PARTICLES: usize = 25_000;
/// Particles closer than this squared distance to the origin won't receive orbital velocity.
/// This prevents division by near-zero and weird behavior for central particles.
pub(crate) const ORBIT_VELOCITY_CUTOFF_DISTANCE_SQ: f32 = 160_000.0;
pub(crate) const MAX_PARTICLE_MASS: f32 = 10.0;
pub(crate) const MIN_PARTICLE_MASS: f32 = 1.0;