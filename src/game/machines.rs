use crate::game::*;
use bevy::state::commands;
use rand::Rng;

#[derive(Component, Clone, Copy, PartialEq)]
pub enum Machine {
    OneToOneCrafter {
        input: Item,
        output: Item,
        cost: i32,
    },
}

#[derive(Component)]
pub struct FollowWithOffset {
    relative_pos: Position,
    entity: Entity,
}

impl Machine {
    pub fn get_description(&self) -> String {
        match self {
            Machine::OneToOneCrafter {
                input,
                output,
                cost,
            } => format!("crafts {} into {}", input.get_name(), output.get_name()),
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            Machine::OneToOneCrafter {
                input,
                output,
                cost,
            } => "Crafter".to_string(),
        }
    }

    pub fn get_cost(&self) -> i32 {
        match self {
            Machine::OneToOneCrafter {
                input,
                output,
                cost,
            } => *cost,
        }
    }

    pub fn get_sprite_view(&self) -> SpriteView {
        match self {
            Machine::OneToOneCrafter {
                input,
                output,
                cost,
            } => SpriteView::OneToOneCrafter,
        }
    }
}

pub fn follow_relative_position(
    mut commands: Commands,
    mut followers: Query<(Entity, &mut Position, &FollowWithOffset), With<FollowWithOffset>>,
    all_objects: Query<(&Position), Without<FollowWithOffset>>,
) {
    for (entity, mut pos, follow) in followers.iter_mut() {
        if let Ok(follow_pos) = all_objects.get(follow.entity) {
            pos.0 = Vec2 {
                x: follow_pos.0.x + follow.relative_pos.0.x,
                y: follow_pos.0.y + follow.relative_pos.0.y,
            };
        } else {
            // parent no longer exists
            commands.entity(entity).despawn();
        }
    }
}

pub fn spawn_one_to_one_crafter(mut commands: Commands, position: Position, crafter: Machine) {
    match crafter {
        Machine::OneToOneCrafter {
            input,
            output,
            cost,
        } => {
            let crafter_width = crafter.get_sprite_view().get_scale().x;
            let crafter_height = crafter.get_sprite_view().get_scale().y;
            let physics_colliders = vec![
                RectanglePhysics {
                    width: 0.0625 * crafter_width,
                    height: 1.0 * crafter_height,
                    offset_x: -0.5625 * crafter_width / 2.0,
                    offset_y: 0.0,
                },
                RectanglePhysics {
                    width: 0.0625 * crafter_width,
                    height: 1.0 * crafter_height,
                    offset_x: 0.5625 * crafter_width / 2.0,
                    offset_y: 0.0,
                },
            ];
            let mut crafter_entity = commands
                .spawn((
                    crafter.get_sprite_view(),
                    position,
                    ColliderCollection(physics_colliders),
                    crafter,
                    Clickable,
                    DragAble,
                ))
                .id();

            // add item icons
            commands.spawn((
                SpriteView::Item {
                    item: input.clone(),
                    ui_element: true,
                },
                Position(Vec2 { x: 0.0, y: 0.0 }),
                FollowWithOffset {
                    relative_pos: Position(Vec2 {
                        x: 0.0,
                        y: crafter_height * 0.3,
                    }),
                    entity: crafter_entity,
                },
            ));
            commands.spawn((
                SpriteView::Item {
                    item: output.clone(),
                    ui_element: true,
                },
                Position(Vec2 { x: 0.0, y: 0.0 }),
                FollowWithOffset {
                    relative_pos: Position(Vec2 {
                        x: 0.0,
                        y: -crafter_height * 0.3,
                    }),
                    entity: crafter_entity,
                },
            ));
        }
        _ => info!("Wrong machine type given"),
    }
}

pub fn one_to_one_crafter(
    mut commands: Commands,
    items: Query<(Entity, &Item, &Position), With<Item>>,
    crafter: Query<(&Machine, &Position), Without<Item>>,
) {
    for (machine, machine_pos) in crafter.iter() {
        match machine {
            Machine::OneToOneCrafter {
                input,
                output,
                cost,
            } => {
                for (item_entity, item, item_pos) in items.iter() {
                    if (machine_pos.0.distance(item_pos.0)
                        < 0.5625 * machine.get_sprite_view().get_scale().x / 2.0)
                    {
                        // item inside machine
                        if (*item == *input) {
                            // spawn product
                            let mut rng = rand::rng();
                            let random_velocity_x = rng.random_range(-50.0..50.0);
                            commands.spawn((
                                SpriteView::Item {
                                    item: output.clone(),
                                    ui_element: false,
                                },
                                output.clone(),
                                Position(Vec2 {
                                    x: machine_pos.0.x,
                                    y: machine_pos.0.y
                                        - machine.get_sprite_view().get_scale().y * 0.6 / 2.0,
                                }),
                                CirclePhysics { radius: 14.0 },
                                Velocity(Vec2 {
                                    x: random_velocity_x,
                                    y: 0.,
                                }),
                            ));
                        }
                        commands.entity(item_entity).despawn();
                    }
                }
            }
            _ => (),
        }
    }
}
