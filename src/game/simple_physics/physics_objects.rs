use bevy::prelude::*;

// Physics objects with a velocity component are considered dynamic, objects without are considered static

#[derive(Component)]
pub struct CirclePhysics {
    pub radius: f32,
}

#[derive(Component)]
pub struct ColliderCollection(pub Vec<RectanglePhysics>);

#[derive(Component)]
pub struct RectanglePhysics {
    pub width: f32,
    pub height: f32,
    pub offset_x: f32,
    pub offset_y: f32,
}

impl Default for RectanglePhysics {
    fn default() -> Self {
        RectanglePhysics {
            width: 10.0,
            height: 10.0,
            offset_x: 0.0,
            offset_y: 0.0,
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Position(pub Vec2);

#[derive(Component)]
pub struct Velocity(pub Vec2);
