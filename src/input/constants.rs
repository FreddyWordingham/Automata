use bevy::prelude::*;

// Key bindings
pub const CAMERA_DOWN: KeyCode = KeyCode::KeyS;
pub const CAMERA_LEFT: KeyCode = KeyCode::KeyA;
pub const CAMERA_RIGHT: KeyCode = KeyCode::KeyD;
pub const CAMERA_UP: KeyCode = KeyCode::KeyW;
pub const CAMERA_ZOOM_IN: KeyCode = KeyCode::KeyE;
pub const CAMERA_ZOOM_OUT: KeyCode = KeyCode::KeyQ;
pub const CAMERA_SPEED_BOOST: KeyCode = KeyCode::ShiftLeft;

pub const CAMERA_PAN_SPEED: f32 = 500.0;
pub const CAMERA_ZOOM_SPEED: f32 = 0.01;
pub const CAMERA_ZOOM_MAX: f32 = 0.1;
pub const CAMERA_SPEED_BOOST_MULTIPLIER: f32 = 4.0;
