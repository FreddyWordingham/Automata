use bevy::prelude::*;

use super::super::constants::*;

pub fn move_camera(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    for (mut transform, mut ortho) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        let mut speed_multiplier = 1.0;
        if keyboard_input.pressed(CAMERA_SPEED_BOOST) {
            speed_multiplier = CAMERA_SPEED_BOOST_MULTIPLIER;
        }

        if keyboard_input.pressed(CAMERA_LEFT) {
            direction -= Vec3::new(CAMERA_PAN_SPEED, 0.0, 0.0);
        }

        if keyboard_input.pressed(CAMERA_RIGHT) {
            direction += Vec3::new(CAMERA_PAN_SPEED, 0.0, 0.0);
        }

        if keyboard_input.pressed(CAMERA_UP) {
            direction += Vec3::new(0.0, CAMERA_PAN_SPEED, 0.0);
        }

        if keyboard_input.pressed(CAMERA_DOWN) {
            direction -= Vec3::new(0.0, CAMERA_PAN_SPEED, 0.0);
        }

        if keyboard_input.pressed(CAMERA_ZOOM_IN) {
            ortho.scale -= CAMERA_ZOOM_SPEED * speed_multiplier;
        }

        if keyboard_input.pressed(CAMERA_ZOOM_OUT) {
            ortho.scale += CAMERA_ZOOM_SPEED * speed_multiplier;
        }

        if ortho.scale < CAMERA_ZOOM_MAX {
            ortho.scale = CAMERA_ZOOM_MAX;
        }

        let z = transform.translation.z;
        transform.translation += time.delta_seconds() * direction * speed_multiplier;

        // Important! We need to restore the Z values when moving the camera around.
        // Bevy has a specific camera setup and this can mess with how our layers are shown.
        transform.translation.z = z;
    }
}
