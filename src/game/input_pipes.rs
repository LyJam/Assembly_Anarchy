use crate::game::*;
use rand::Rng;

pub const INPUT_PIPE_POS1: Vec2 = Vec2::new(-600., 350.);
pub const INPUT_PIPE_POS2: Vec2 = Vec2::new(-300., 350.);
pub const INPUT_PIPE_POS3: Vec2 = Vec2::new(0., 350.);
pub const INPUT_PIPE_POS4: Vec2 = Vec2::new(300., 350.);
pub const INPUT_PIPE_POS5: Vec2 = Vec2::new(600., 350.);

#[derive(Component)]
pub struct InputPipe {
    pub item: Item,
    pub spawn_rate: f32, // spawns per second
    pub time_elapsed: f32,
}

pub fn spawn_item(
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
                Position(Vec2 {
                    x: pipe_position.0.x,
                    y: pipe_position.0.y + 40.0,
                }),
                CirclePhysics { radius: 16.0 },
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
