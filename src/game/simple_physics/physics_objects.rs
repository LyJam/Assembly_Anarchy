use bevy::prelude::*;

// Physics objects with a velocity component are considered dynamic, objects without are considered static

#[derive(Component)]
pub struct CirclePhysics {
    pub radius: f32,
}

#[derive(Component)]
pub struct RectanglePhysics {
    pub width: f32,
    pub height: f32,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Position(pub Vec2);

#[derive(Component)]
pub struct Velocity(pub Vec2);
