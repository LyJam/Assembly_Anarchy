use crate::game::*;
use bevy::color::palettes::basic::*;

use rand::Rng;

pub const INPUT_PIPE_POS1: Vec2 = Vec2::new(-600., 350.);
pub const INPUT_PIPE_POS2: Vec2 = Vec2::new(-300., 350.);
pub const INPUT_PIPE_POS3: Vec2 = Vec2::new(0., 350.);
pub const INPUT_PIPE_POS4: Vec2 = Vec2::new(300., 350.);
pub const INPUT_PIPE_POS5: Vec2 = Vec2::new(600., 350.);

pub const OUTPUT_PIPE_POS1: Vec2 = Vec2::new(-600., -350.);
pub const OUTPUT_PIPE_POS2: Vec2 = Vec2::new(-300., -350.);
pub const OUTPUT_PIPE_POS3: Vec2 = Vec2::new(0., -350.);
pub const OUTPUT_PIPE_POS4: Vec2 = Vec2::new(300., -350.);
pub const OUTPUT_PIPE_POS5: Vec2 = Vec2::new(600., -350.);

#[derive(Component)]
pub struct InputPipe {
    pub item: Item,
    pub spawn_rate: f32, // spawns per second
    pub time_elapsed: f32,
}

#[derive(Component)]
pub struct OutputPipe {
    pub item: Item,
}

pub fn on_add_output_pipe(
    trigger: Trigger<OnAdd, OutputPipe>,
    views: Query<(&OutputPipe, &Position, &SpriteView)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let entity = trigger.entity();
    if let Ok((view, pos, sprite_view)) = views.get(entity) {
        // Add the ledges of the pipe.
        // These values are based on the layout of the sprite and determined experimentally. An editor for bevy would be nice :p
        let half_diameter = sprite_view.get_scale().x / 2.0;
        // right side
        commands.spawn((
            Transform::from_xyz(
                0.56 * half_diameter + pos.0.x,
                -0.50 * half_diameter + pos.0.y,
                20.0,
            )
            .with_scale(Vec3::splat(1.0 * half_diameter)),
            Position(Vec2 {
                x: 0.56 * half_diameter + pos.0.x,
                y: -0.50 * half_diameter + pos.0.y,
            }),
            RectanglePhysics {
                width: 0.2 * half_diameter,
                height: 1.0 * half_diameter,
            },
        ));
        //left side
        commands.spawn((
            Transform::from_xyz(
                -0.56 * half_diameter + pos.0.x,
                -0.50 * half_diameter + pos.0.y,
                20.0,
            )
            .with_scale(Vec3::splat(1.0 * half_diameter)),
            Position(Vec2 {
                x: -0.56 * half_diameter + pos.0.x,
                y: -0.50 * half_diameter + pos.0.y,
            }),
            RectanglePhysics {
                width: 0.2 * half_diameter,
                height: 1.0 * half_diameter,
            },
        ));
    }
}

pub fn output_pipe_consume_item(
    mut commands: Commands,
    items: Query<(Entity, &Item, &Position)>,
    pipes: Query<(&OutputPipe, &Position, &SpriteView)>,
) {
    for (pipe, pipe_pos, pipe_view) in pipes.iter() {
        let half_diameter = pipe_view.get_scale().y / 2.0;
        let pipe_collection_point = Vec2 {
            x: pipe_pos.0.x,
            y: pipe_pos.0.y - half_diameter,
        };
        let collection_diameter = 0.5 * half_diameter;
        for (item_entity, item, item_pos) in items.iter() {
            if (item_pos.0.distance(pipe_collection_point) < collection_diameter) {
                if (*item == pipe.item) {
                    info!("item collected! (todo: increase score/money here)");
                    commands.entity(item_entity).despawn();
                }
            }
        }
    }
}

pub fn input_pipe_spawn_item(
    mut commands: Commands,
    mut pipes: Query<(Entity, &mut InputPipe, &Position)>,
    time: Res<Time>,
) {
    for (pipe_entity, mut input_pipe, pipe_position) in pipes.iter_mut() {
        input_pipe.time_elapsed += time.delta_secs();

        // Calculate the interval between spawns. Avoid division by zero if spawn_rate is 0.
        let spawn_interval = if input_pipe.spawn_rate > 0.0 {
            1.0 / input_pipe.spawn_rate
        } else {
            f32::MAX
        };

        // Check if enough time has elapsed to spawn an item.
        while input_pipe.time_elapsed >= spawn_interval {
            let mut rng = rand::rng();
            let random_velocity_x = rng.random_range(-50.0..50.0);
            commands.spawn((
                SpriteView::Item {
                    item: input_pipe.item.clone(),
                },
                input_pipe.item.clone(),
                Position(Vec2 {
                    x: pipe_position.0.x,
                    y: pipe_position.0.y + 40.0,
                }),
                CirclePhysics { radius: 14.0 },
                Velocity(Vec2 {
                    x: random_velocity_x,
                    y: 0.,
                }),
            ));

            // Subtract the spawn interval from time_elapsed. This is crucial for accuracy.
            // Don't just reset to 0, in case `time_elapsed` accumulated much more than `spawn_interval`.
            input_pipe.time_elapsed -= spawn_interval;
        }
    }
}
