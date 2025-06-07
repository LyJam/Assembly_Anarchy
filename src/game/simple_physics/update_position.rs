use crate::game::*;
use bevy::prelude::*;

const GRAVITY_ACCELERATION: f32 = -9.8 * 80.0;

pub fn add_gravity(mut query: Query<&mut Velocity>, time: Res<Time>) {
    for mut vel in query.iter_mut() {
        // Apply gravity by adding to the y-component of velocity
        vel.0.y += GRAVITY_ACCELERATION * time.delta_secs();
    }
}

pub fn update_position(mut query: Query<(&mut Position, &Velocity)>, time: Res<Time>) {
    for (mut pos, vel) in query.iter_mut() {
        // Update position based on velocity and delta time
        pos.0 += vel.0 * time.delta_secs();
    }
}

pub fn on_changed_position(
    mut changed_pos: Query<(&Position, &SpriteView, &mut Transform), Changed<Position>>,
) {
    for (pos, view, mut transform) in changed_pos.iter_mut() {
        transform.translation = Vec3::new(pos.0.x, pos.0.y, view.get_layer());
    }
}
