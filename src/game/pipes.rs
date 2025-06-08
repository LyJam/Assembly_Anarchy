use crate::game::*;
use bevy::color::palettes::basic::*;

use rand::Rng;

// left most pipe (POS1) is blocked by UI element
pub const INPUT_PIPE_POS2: Vec2 = Vec2::new(-300., 400.);
pub const INPUT_PIPE_POS3: Vec2 = Vec2::new(0., 400.);
pub const INPUT_PIPE_POS4: Vec2 = Vec2::new(300., 400.);
pub const INPUT_PIPE_POS5: Vec2 = Vec2::new(600., 400.);

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
    pub cost: i32, // cost per item spawned
    pub enabled: bool,
}

#[derive(Component)]
pub struct OutputPipe {
    pub item: Item,
    pub reward: i32, // money per item returned
}

pub fn on_add_input_pipe(
    trigger: Trigger<OnAdd, InputPipe>,
    pipes: Query<(&InputPipe, &Position)>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let entity = trigger.entity();
    if let Ok((pipe, pos)) = pipes.get(entity) {
        // add the item icon
        commands.spawn((
            SpriteView::Item {
                item: pipe.item.clone(),
                ui_element: true,
            },
            Position(Vec2 {
                x: pos.0.x,
                y: pos.0.y + 15.,
            }),
        ));

        // add reward text
        commands.spawn((
            Text::new(format!("{:?}", pipe.cost)),
            TextFont {
                font: asset_server.load("Fonts/CyberpunkCraftpixPixel.otf"),
                font_size: 40.,
                ..default()
            },
            TextColor(Color::srgb(255.0 / 255.0, 130.0 / 255.0, 130.0 / 255.0)),
            TextLayout::new_with_justify(JustifyText::Center),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(35.0),
                left: Val::Px(pos.0.x + 800. + 10.),
                ..default()
            },
            Position(Vec2 { x: 0., y: 0. }), // de-spawn marker
        ));
    }
}

pub fn on_add_output_pipe(
    trigger: Trigger<OnAdd, OutputPipe>,
    pipes: Query<(&OutputPipe, &Position, &SpriteView)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let entity = trigger.entity();
    if let Ok((pipe, pos, sprite_view)) = pipes.get(entity) {
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
            ColliderCollection(vec![RectanglePhysics {
                width: 0.2 * half_diameter,
                height: 1.0 * half_diameter,
                ..Default::default()
            }]),
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
            ColliderCollection(vec![RectanglePhysics {
                width: 0.2 * half_diameter,
                height: 1.0 * half_diameter,
                ..Default::default()
            }]),
        ));

        // add the item icon
        commands.spawn((
            SpriteView::Item {
                item: pipe.item.clone(),
                ui_element: true,
            },
            Position(Vec2 {
                x: pos.0.x,
                y: pos.0.y - 50.0,
            }),
        ));

        // add reward text
        commands.spawn((
            Text::new(format!("{:?}", pipe.reward)),
            TextFont {
                font: asset_server.load("Fonts/CyberpunkCraftpixPixel.otf"),
                font_size: 40.,
                ..default()
            },
            TextColor(Color::srgb(255.0 / 255.0, 215.0 / 255.0, 0.0)),
            TextLayout::new_with_justify(JustifyText::Center),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                left: Val::Px(pos.0.x + 800. + 10.),
                ..default()
            },
            Position(Vec2 { x: 0., y: 0. }), // de-spawn marker
        ));
    }
}

pub fn output_pipe_consume_item(
    mut commands: Commands,
    items: Query<(Entity, &Item, &Position)>,
    pipes: Query<(&OutputPipe, &Position, &SpriteView)>,
    mut money: ResMut<CurrentMoney>,
    asset_server: Res<AssetServer>,
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
                    money.0 += pipe.reward;
                    commands.entity(item_entity).despawn();
                    let music_handle = asset_server.load::<AudioSource>("Music/coin.ogg");

                    // Spawn an entity to play the music
                    commands.spawn((
                        AudioPlayer(music_handle),
                        PlaybackSettings {
                            mode: bevy::audio::PlaybackMode::Once,
                            ..default()
                        },
                    ));
                }
            }
        }
    }
}

pub fn toggle_input_pipe(
    clicked_resource: Res<JustClicked>,
    mut pipes: Query<(Entity, &mut InputPipe)>,
) {
    for (pipe_entity, mut pipe) in pipes.iter_mut() {
        if let Some(clicked_entity) = clicked_resource.0 {
            if (clicked_entity == pipe_entity) {
                pipe.enabled = !pipe.enabled;
            }
        }
    }
}

pub fn input_pipe_spawn_item(
    mut commands: Commands,
    mut pipes: Query<(Entity, &mut InputPipe, &Position)>,
    time: Res<Time>,
    mut money: ResMut<CurrentMoney>,
) {
    for (pipe_entity, mut input_pipe, pipe_position) in pipes.iter_mut() {
        if (!input_pipe.enabled) {
            return;
        }

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
                    ui_element: false,
                },
                input_pipe.item.clone(),
                Position(Vec2 {
                    x: pipe_position.0.x,
                    y: pipe_position.0.y,
                }),
                CirclePhysics { radius: 14.0 },
                Velocity(Vec2 {
                    x: random_velocity_x,
                    y: 0.,
                }),
            ));

            // pay the price for the item
            money.0 -= input_pipe.cost;

            // Subtract the spawn interval from time_elapsed. This is crucial for accuracy.
            // Don't just reset to 0, in case `time_elapsed` accumulated much more than `spawn_interval`.
            input_pipe.time_elapsed -= spawn_interval;
        }
    }
}
