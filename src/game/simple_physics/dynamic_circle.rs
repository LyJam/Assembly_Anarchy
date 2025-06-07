use crate::game::*;
use bevy::prelude::*;

const RESTITUTION: f32 = 0.7; // Coefficient of restitution: 0.0 (perfectly inelastic) to 1.0 (perfectly elastic)

pub fn collision_with_static_circles(
    mut dynamic_circles_query: Query<(&mut Position, &mut Velocity, &CirclePhysics)>,
    static_circles_query: Query<(&Position, &CirclePhysics), Without<Velocity>>,
) {
    for (mut dyn_pos, mut dyn_vel, dyn_circle) in dynamic_circles_query.iter_mut() {
        for (static_pos, static_circle) in static_circles_query.iter() {
            let distance_vec = dyn_pos.0 - static_pos.0;
            let distance = distance_vec.length();
            let sum_radii = dyn_circle.radius + static_circle.radius;

            // Check for collision
            if distance < sum_radii {
                // Collision detected!

                // 1. Separation: Move the dynamic circle out of the static circle
                let overlap = sum_radii - distance;
                // Avoid division by zero if circles are perfectly overlapping at the same point
                let normal = if distance == 0.0 {
                    Vec2::X // Default normal if centers are identical
                } else {
                    distance_vec.normalize()
                };

                // Push the dynamic circle away along the normal
                dyn_pos.0 += normal * overlap;

                // 2. Reflection: Adjust the dynamic circle's velocity
                // Calculate the component of velocity along the collision normal
                let velocity_along_normal = dyn_vel.0.dot(normal);

                // Only reflect if objects are moving towards each other
                if velocity_along_normal < 0.0 {
                    // Calculate the impulse to apply
                    let impulse = -(1.0 + RESTITUTION) * velocity_along_normal;

                    // Apply the impulse to the velocity
                    dyn_vel.0 += normal * impulse;
                }
            }
        }
    }
}

pub fn collision_with_static_rectangles(
    mut dynamic_circles_query: Query<(&mut Position, &mut Velocity, &CirclePhysics)>,
    static_rectangles_query: Query<(&Position, &RectanglePhysics), Without<Velocity>>,
) {
    for (mut dyn_pos, mut dyn_vel, dyn_circle) in dynamic_circles_query.iter_mut() {
        for (rect_pos, rect) in static_rectangles_query.iter() {
            // Calculate rectangle's half-dimensions
            let half_width = rect.width / 2.0;
            let half_height = rect.height / 2.0;

            // Find the closest point on the rectangle to the circle's center
            let closest_x = dyn_pos
                .0
                .x
                .clamp(rect_pos.0.x - half_width, rect_pos.0.x + half_width);
            let closest_y = dyn_pos
                .0
                .y
                .clamp(rect_pos.0.y - half_height, rect_pos.0.y + half_height);

            let closest_point = Vec2::new(closest_x, closest_y);

            // Vector from the closest point on rectangle to circle center
            let distance_vec = dyn_pos.0 - closest_point;
            let distance = distance_vec.length();

            // Check for collision
            if distance < dyn_circle.radius {
                // Collision detected!

                // 1. Separation: Move the dynamic circle out of the rectangle
                let overlap = dyn_circle.radius - distance;
                let normal: Vec2;

                // Handle the edge case where the circle's center is exactly at the closest point
                // (i.e., the circle's center is inside the rectangle).
                // In this scenario, `distance_vec` would be zero, making normalization impossible.
                // We determine the normal by finding the axis with the minimum overlap.
                if distance == 0.0 {
                    // Calculate penetration depths on each axis
                    let dx_min = dyn_pos.0.x - (rect_pos.0.x - half_width); // Distance to left edge of rect
                    let dx_max = (rect_pos.0.x + half_width) - dyn_pos.0.x; // Distance to right edge of rect
                    let dy_min = dyn_pos.0.y - (rect_pos.0.y - half_height); // Distance to bottom edge of rect
                    let dy_max = (rect_pos.0.y + half_height) - dyn_pos.0.y; // Distance to top edge of rect

                    // Find the smallest penetration depth
                    let min_x_overlap = dx_min.min(dx_max);
                    let min_y_overlap = dy_min.min(dy_max);

                    if min_x_overlap < min_y_overlap {
                        // Push along X-axis
                        normal = if dx_min < dx_max {
                            Vec2::NEG_X
                        } else {
                            Vec2::X
                        };
                    } else {
                        // Push along Y-axis
                        normal = if dy_min < dy_max {
                            Vec2::NEG_Y
                        } else {
                            Vec2::Y
                        };
                    }
                    // Since distance was 0, it means the circle is fully inside.
                    // We push it out by its radius plus a small margin (or just radius for simplicity)
                    // This is a heuristic for when the circle spawns inside.
                    dyn_pos.0 += normal * dyn_circle.radius;
                } else {
                    normal = distance_vec.normalize();
                    dyn_pos.0 += normal * overlap;
                }

                // 2. Reflection: Adjust the dynamic circle's velocity
                let velocity_along_normal = dyn_vel.0.dot(normal);

                // Only reflect if objects are moving towards each other
                if velocity_along_normal < 0.0 {
                    let impulse = -(1.0 + RESTITUTION) * velocity_along_normal;
                    dyn_vel.0 += normal * impulse;
                }
            }
        }
    }
}
