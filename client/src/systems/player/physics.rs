use bevy::prelude::*;

use crate::components::{
    player::{FollowCamera, Grounded, Player, Velocity},
    world::Collidable,
};

const PLAYER_SPEED: f32 = 15.0; // Increased speed for larger corridors
const GRAVITY: f32 = -9.8;
const JUMP_FORCE: f32 = 5.5;
const PLAYER_RADIUS: f32 = 0.5; // Adjusted for 6-unit wide corridors
const WALL_SIZE: f32 = 3.0; // Updated to match new wall half-size (6-unit scale)

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_q: Query<(&mut Transform, &mut Velocity), (With<Player>, Without<FollowCamera>)>,
    camera_q: Query<&Transform, (With<FollowCamera>, Without<Player>)>,
    collidable_q: Query<&Transform, (With<Collidable>, Without<Player>)>,
    time: Res<Time>,
) {
    // Get camera's yaw rotation (we only care about Y-axis rotation)
    let camera_transform = if let Ok(transform) = camera_q.single() {
        transform
    } else {
        return;
    };

    for (mut transform, _velocity) in player_q.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.z -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.z += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
            // Transform the direction vector by the camera's yaw rotation
            let camera_rotation =
                Quat::from_rotation_y(camera_transform.rotation.to_euler(EulerRot::YXZ).0);
            direction = camera_rotation * direction;
        }

        let move_delta = direction * PLAYER_SPEED * time.delta_secs();

        // Try to move in each direction separately to allow wall sliding
        let current_pos = transform.translation;

        // Try X movement first
        let new_x = current_pos + Vec3::new(move_delta.x, 0.0, 0.0);
        if !is_position_blocked(new_x, &collidable_q) {
            transform.translation.x = new_x.x;
        }

        // Try Z movement
        let new_z = transform.translation + Vec3::new(0.0, 0.0, move_delta.z);
        if !is_position_blocked(new_z, &collidable_q) {
            transform.translation.z = new_z.z;
        }
    }
}

pub fn apply_gravity(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &mut Transform, &mut Grounded), With<Player>>,
) {
    for (mut velocity, mut transform, mut grounded) in query.iter_mut() {
        if transform.translation.y <= 2.0 {
            transform.translation.y = 2.0;
            velocity.linear_velocity.y = 0.0;
            grounded.0 = true;
        } else {
            velocity.linear_velocity.y += GRAVITY * time.delta_secs();
            grounded.0 = false;
        }

        // Apply vertical velocity
        transform.translation.y += velocity.linear_velocity.y * time.delta_secs();
    }
}

pub fn handle_jumping(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &Grounded), With<Player>>,
) {
    for (mut velocity, grounded) in query.iter_mut() {
        if grounded.0 && keyboard_input.just_pressed(KeyCode::Space) {
            velocity.linear_velocity.y = JUMP_FORCE;
        }
    }
}

pub fn handle_collisions(
    mut player_q: Query<&mut Transform, With<Player>>,
    collidable_q: Query<&Transform, (With<Collidable>, Without<Player>)>,
) {
    // Simple collision system that just ensures player doesn't get stuck
    for mut player_transform in player_q.iter_mut() {
        if is_position_blocked(player_transform.translation, &collidable_q) {
            // If somehow stuck, try to find a nearby valid position
            let pos = player_transform.translation;
            let offsets = [
                Vec3::new(0.1, 0.0, 0.0),
                Vec3::new(-0.1, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.1),
                Vec3::new(0.0, 0.0, -0.1),
            ];

            for offset in offsets.iter() {
                let test_pos = pos + *offset;
                if !is_position_blocked(test_pos, &collidable_q) {
                    player_transform.translation = test_pos;
                    break;
                }
            }
        }
    }
}

// Simplified collision detection
fn is_position_blocked(
    player_pos: Vec3,
    collidable_q: &Query<&Transform, (With<Collidable>, Without<Player>)>,
) -> bool {
    for collidable_transform in collidable_q.iter() {
        let diff = player_pos - collidable_transform.translation;
        let distance_x = diff.x.abs();
        let distance_z = diff.z.abs();

        // Check if player overlaps with wall
        if distance_x < (PLAYER_RADIUS + WALL_SIZE) && distance_z < (PLAYER_RADIUS + WALL_SIZE) {
            return true;
        }
    }

    false
}
