use bevy::app::PluginGroupBuilder;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::input::keyboard::KeyCode;

struct ControlsPlugin;

pub fn camera_pan_keyboard(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera2d>>,
    time: Res<Time>,
) {
    let mut direction = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    if direction != Vec2::ZERO {
        let delta = direction.normalize() * 500.0 * time.delta_secs(); // Adjust speed as needed
        for mut transform in query.iter_mut() {
            let scale = transform.scale.truncate(); // Respect zoom level
            transform.translation.x += delta.x * scale.x;
            transform.translation.y += delta.y * scale.y;
        }
    }
}

const MIN_ZOOM: f32 = 0.1;
const MAX_ZOOM: f32 = 10.0;
const ZOOM_SENSITIVITY: f32 = 0.1;


pub fn camera_zoom(
    mut scroll_evr: EventReader<MouseWheel>,
    mut query: Query<&mut Projection, With<Camera2d>>,
) {
    for ev in scroll_evr.read() {
        let zoom_delta = match ev.unit {
            MouseScrollUnit::Line => ev.y,
            MouseScrollUnit::Pixel => ev.y / 100.0, // Adjust this value as needed
        };

        for mut projection in query.iter_mut() {
            if let Projection::Orthographic(ref mut ortho) = *projection {
                let zoom_factor = 1.0 - zoom_delta * 0.1;
                ortho.scale *= zoom_factor.clamp(0.9, 1.1);
                ortho.scale = ortho.scale.clamp(0.1, 100.0); // Prevent extreme zoom
            }
        }
    }
}

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (camera_pan_keyboard, camera_zoom));
    }
    
}



