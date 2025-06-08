use crate::game::*;
use bevy::{state::commands, utils::HashMap};

// A resource to track the current level number.
#[derive(Resource, Default)]
pub struct CurrentLevel(pub u32);

// A resource that holds a map of level numbers to their spawn functions.
#[derive(Resource)]
pub struct LevelRegistry(
    pub HashMap<u32, fn(Commands, ResMut<CurrentMoney>, ResMut<MoneyGoal>, Res<AssetServer>) -> ()>,
); // A map from level number to a spawn function

impl Default for LevelRegistry {
    fn default() -> Self {
        let mut map = HashMap::new();
        // Register your level spawn functions here:
        map.insert(
            0u32,
            load_level_0 as fn(Commands, ResMut<CurrentMoney>, ResMut<MoneyGoal>, Res<AssetServer>),
        );
        map.insert(
            1u32,
            load_level_1 as fn(Commands, ResMut<CurrentMoney>, ResMut<MoneyGoal>, Res<AssetServer>),
        );
        map.insert(
            2u32,
            load_level_2 as fn(Commands, ResMut<CurrentMoney>, ResMut<MoneyGoal>, Res<AssetServer>),
        );
        map.insert(
            3u32,
            load_level_3 as fn(Commands, ResMut<CurrentMoney>, ResMut<MoneyGoal>, Res<AssetServer>),
        );
        map.insert(
            4u32,
            load_level_4 as fn(Commands, ResMut<CurrentMoney>, ResMut<MoneyGoal>, Res<AssetServer>),
        );
        map.insert(
            5u32,
            load_level_5 as fn(Commands, ResMut<CurrentMoney>, ResMut<MoneyGoal>, Res<AssetServer>),
        );
        map.insert(
            6u32,
            load_level_6 as fn(Commands, ResMut<CurrentMoney>, ResMut<MoneyGoal>, Res<AssetServer>),
        );
        map.insert(
            7u32,
            load_level_7 as fn(Commands, ResMut<CurrentMoney>, ResMut<MoneyGoal>, Res<AssetServer>),
        );
        map.insert(
            8u32,
            load_level_8 as fn(Commands, ResMut<CurrentMoney>, ResMut<MoneyGoal>, Res<AssetServer>),
        );
        map.insert(
            9u32,
            load_level_9 as fn(Commands, ResMut<CurrentMoney>, ResMut<MoneyGoal>, Res<AssetServer>),
        );
        LevelRegistry(map)
    }
}

#[derive(Resource)]
pub struct CurrentMoney(pub i32);

// money goal = win condition
#[derive(Resource)]
pub struct MoneyGoal(pub i32);

#[derive(Resource)]
pub struct LevelWon(pub bool);

#[derive(Resource)]
pub struct LevelLost(pub bool);

pub fn load_initial_level(
    mut commands: Commands,
    level_registry: Res<LevelRegistry>,
    current_level: Res<CurrentLevel>,
    mut money: ResMut<CurrentMoney>,
    mut money_goal: ResMut<MoneyGoal>,
    asset_server: Res<AssetServer>,
) {
    if let Some(spawn_fn) = level_registry.0.get(&current_level.0) {
        spawn_fn(commands, money, money_goal, asset_server);
    } else {
        error!(
            "No spawn function registered for initial level {}",
            current_level.0
        );
    }
}

pub fn level_management(
    mut current_money: ResMut<CurrentMoney>,
    mut money_goal: ResMut<MoneyGoal>,
    mut level_won: ResMut<LevelWon>,
    mut level_lost: ResMut<LevelLost>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut current_level: ResMut<CurrentLevel>,
    level_registry: Res<LevelRegistry>,
    level_entities_query: Query<Entity, With<Position>>, // Query needed for de-spawn
    mut selected_tool: ResMut<SelectedTool>,
) {
    if (current_money.0 >= money_goal.0 && !level_won.0 && !level_lost.0) {
        level_won.0 = true;

        let mut complete_text = Text::new("Level Complete!");
        let mut subtext = Text::new("Press Any Key To Continue");
        if current_level.0 == (level_registry.0.iter().count() - 1) as u32 {
            complete_text = Text::new("You won!");
            subtext = Text::new("please leave a rating :)");
        }
        commands.spawn((
            complete_text,
            TextFont {
                font: asset_server.load("Fonts/CyberpunkCraftpixPixel.otf"),
                font_size: 100.,
                ..default()
            },
            TextColor(Color::srgb(255.0 / 255.0, 215.0 / 255.0, 0.0)),
            TextLayout::new_with_justify(JustifyText::Center),
            BoxShadow {
                x_offset: Val::Percent(0.),
                y_offset: Val::Percent(0.),
                blur_radius: Val::Percent(2.),
                ..Default::default()
            },
            Node {
                margin: UiRect {
                    top: Val::Percent(25.0),
                    ..Default::default()
                },
                width: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
            Position(Vec2 { x: 0., y: 0. }),
        ));
        commands.spawn((
            subtext,
            TextFont {
                font: asset_server.load("Fonts/CyberpunkCraftpixPixel.otf"),
                font_size: 30.,
                ..default()
            },
            TextColor(Color::srgb(255.0 / 255.0, 215.0 / 255.0, 0.0)),
            TextLayout::new_with_justify(JustifyText::Center),
            Node {
                margin: UiRect {
                    top: Val::Percent(33.0),
                    ..Default::default()
                },
                width: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
            Position(Vec2 { x: 0., y: 0. }),
        ));
    }

    if (current_money.0 < 0 && !level_won.0 && !level_lost.0) {
        level_lost.0 = true;

        commands.spawn((
            Text::new("bankruptcy!"),
            TextFont {
                font: asset_server.load("Fonts/CyberpunkCraftpixPixel.otf"),
                font_size: 100.,
                ..default()
            },
            TextColor(Color::srgb(255.0 / 255.0, 130.0 / 255.0, 130.0 / 255.0)),
            TextLayout::new_with_justify(JustifyText::Center),
            BoxShadow {
                x_offset: Val::Percent(0.),
                y_offset: Val::Percent(0.),
                blur_radius: Val::Percent(2.),
                ..Default::default()
            },
            Node {
                margin: UiRect {
                    top: Val::Percent(25.0),
                    ..Default::default()
                },
                width: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
            Position(Vec2 { x: 0., y: 0. }),
        ));
        commands.spawn((
            Text::new("You ran out of money \n press any key to try again"),
            TextFont {
                font: asset_server.load("Fonts/CyberpunkCraftpixPixel.otf"),
                font_size: 30.,
                ..default()
            },
            TextColor(Color::srgb(211.0 / 255.0, 211.0 / 255.0, 211.0 / 255.0)),
            TextLayout::new_with_justify(JustifyText::Center),
            Node {
                margin: UiRect {
                    top: Val::Percent(33.0),
                    ..Default::default()
                },
                width: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
            Position(Vec2 { x: 0., y: 0. }),
        ));
    }

    if (current_level.0 == 0) {
        level_won.0 = true;
    }

    if (level_won.0 || level_lost.0) {
        if (keyboard_input.get_pressed().count() > 0) {
            if (level_won.0) {
                info!("Level completed! Loading next level...");
                current_level.0 += 1; // Increment the level
            }
            level_won.0 = false;
            level_lost.0 = false;
            current_money.0 = 123;
            money_goal.0 = 1234;
            selected_tool.0 = Tools::Mouse;

            // Despawn all entities from the current level
            for entity in level_entities_query.iter() {
                commands.entity(entity).despawn_recursive();
            }

            // Check if the next level exists in the registry
            if let Some(spawn_fn) = level_registry.0.get(&current_level.0) {
                info!("Loading Level: {}", current_level.0);
                spawn_fn(commands, current_money, money_goal, asset_server); // Call the spawn function for the new level
            } else {
                // No more levels defined, handle "Game Over" or loop back to level 0
                info!(
                    "No more levels found. Current level: {}. Game Over!",
                    current_level.0
                );
            }
        }
    }
}

pub fn load_level_0(
    mut commands: Commands,
    mut money: ResMut<CurrentMoney>,
    mut money_goal: ResMut<MoneyGoal>,
    asset_server: Res<AssetServer>,
) {
    money.0 = 123;
    money_goal.0 = 1234;

    commands.spawn((
        SpriteView::BackgroundCity,
        Position(Vec2 { x: 0.0, y: 0.0 }),
    ));

    /* explainer level */
    commands.spawn((
        Text::new("Assembly Anarchy"),
        TextFont {
            font: asset_server.load("Fonts/CyberpunkCraftpixPixel.otf"),
            font_size: 100.,
            ..default()
        },
        TextColor(Color::srgb(255.0 / 255.0, 215.0 / 255.0, 0.0)),
        TextLayout::new_with_justify(JustifyText::Center),
        BoxShadow {
            x_offset: Val::Percent(0.),
            y_offset: Val::Percent(0.),
            blur_radius: Val::Percent(2.),
            ..Default::default()
        },
        Node {
            margin: UiRect {
                top: Val::Percent(20.0),
                ..Default::default()
            },
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position_type: PositionType::Absolute,
            ..default()
        },
        Position(Vec2 { x: 0., y: 0. }),
    ));

    commands.spawn((
        Text::new("Assemble the correct items from the given inputs to earn money \n Good luck and have fun! \n\n press any key to continue..."),
        TextFont {
            font: asset_server.load("Fonts/CyberpunkCraftpixPixel.otf"),
            font_size: 30.,
            ..default()
        },
        TextColor(Color::srgb(211.0 / 255.0, 211.0 / 255.0, 211.0 / 255.0)),
        TextLayout::new_with_justify(JustifyText::Center),
        BoxShadow {
            x_offset: Val::Percent(0.),
            y_offset: Val::Percent(0.),
            blur_radius: Val::Percent(5.),
            spread_radius: Val::Percent(100.),
            ..Default::default()
        },
        Node {
            margin: UiRect {
                top: Val::Percent(30.0),
                ..Default::default()
            },
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position_type: PositionType::Absolute,
            ..default()
        },
        Position(Vec2 { x: 0., y: 0. }),
    ));
}

pub fn load_level_1(
    mut commands: Commands,
    mut money: ResMut<CurrentMoney>,
    mut money_goal: ResMut<MoneyGoal>,
    asset_server: Res<AssetServer>,
) {
    money.0 = 100;
    money_goal.0 = 120;

    commands.spawn((
        SpriteView::InputPipe,
        Position(INPUT_PIPE_POS3),
        Clickable,
        InputPipe {
            item: Item::Bolt,
            spawn_rate: 5.,
            time_elapsed: 0.,
            enabled: false,
            cost: 1,
        },
    ));

    commands.spawn((
        SpriteView::OutputPipe,
        OutputPipe {
            item: Item::Bolt,
            reward: 5,
        },
        Position(OUTPUT_PIPE_POS3),
    ));

    commands.spawn((
        SpriteView::BackgroundIndustry,
        Position(Vec2 { x: 0.0, y: 0.0 }),
    ));

    // user click indicator (mini tutorial)
    commands.spawn((
        SpriteView::CursorPointLeft,
        Position(Vec2 {
            x: INPUT_PIPE_POS3.x + 125.,
            y: INPUT_PIPE_POS3.y,
        }),
    ));
}

pub fn load_level_2(
    mut commands: Commands,
    mut money: ResMut<CurrentMoney>,
    mut money_goal: ResMut<MoneyGoal>,
    asset_server: Res<AssetServer>,
) {
    money.0 = 100;
    money_goal.0 = 120;

    setup_mouse_button(commands.reborrow());
    setup_draw_button(commands.reborrow());

    commands.spawn((
        SpriteView::InputPipe,
        Position(INPUT_PIPE_POS3),
        Clickable,
        InputPipe {
            item: Item::Bolt,
            spawn_rate: 5.,
            time_elapsed: 0.,
            enabled: false,
            cost: 1,
        },
    ));

    commands.spawn((
        SpriteView::OutputPipe,
        OutputPipe {
            item: Item::Bolt,
            reward: 5,
        },
        Position(OUTPUT_PIPE_POS5),
    ));

    commands.spawn((
        SpriteView::BackgroundIndustry2,
        Position(Vec2 { x: 0.0, y: 0.0 }),
    ));

    // user click indicator (mini tutorial)
    commands.spawn((
        SpriteView::CursorPointLeft,
        Position(Vec2 {
            x: BUTTON_DRAW_POS.x + 100.,
            y: BUTTON_DRAW_POS.y,
        }),
    ));
}

pub fn load_level_3(
    mut commands: Commands,
    mut money: ResMut<CurrentMoney>,
    mut money_goal: ResMut<MoneyGoal>,
    asset_server: Res<AssetServer>,
) {
    money.0 = 200;
    money_goal.0 = 300;

    setup_mouse_button(commands.reborrow());
    setup_draw_button(commands.reborrow());

    commands.spawn((
        SpriteView::InputPipe,
        Position(INPUT_PIPE_POS3),
        Clickable,
        InputPipe {
            item: Item::Steel,
            spawn_rate: 5.,
            time_elapsed: 0.,
            enabled: false,
            cost: 2,
        },
    ));

    spawn_machine_window(
        Position(MACHINE_BUTTON_1),
        Machine::OneToOneCrafter {
            input: Item::Steel,
            output: Item::Bolt,
            cost: 100,
        },
        commands.reborrow(),
        &asset_server,
    );

    commands.spawn((
        SpriteView::OutputPipe,
        OutputPipe {
            item: Item::Bolt,
            reward: 10,
        },
        Position(OUTPUT_PIPE_POS3),
    ));

    commands.spawn((
        SpriteView::BackgroundIndustry,
        Position(Vec2 { x: 0.0, y: 0.0 }),
    ));

    // user click indicator (mini tutorial)
    commands.spawn((
        SpriteView::CursorPointRight,
        Position(Vec2 {
            x: MACHINE_BUTTON_1.x + -210.,
            y: MACHINE_BUTTON_1.y,
        }),
    ));
}

pub fn load_level_4(
    mut commands: Commands,
    mut money: ResMut<CurrentMoney>,
    mut money_goal: ResMut<MoneyGoal>,
    asset_server: Res<AssetServer>,
) {
    money.0 = 200;
    money_goal.0 = 300;

    setup_mouse_button(commands.reborrow());
    setup_draw_button(commands.reborrow());

    commands.spawn((
        SpriteView::InputPipe,
        Position(INPUT_PIPE_POS4),
        Clickable,
        InputPipe {
            item: Item::Iron,
            spawn_rate: 15.,
            time_elapsed: 0.,
            enabled: false,
            cost: 2,
        },
    ));

    spawn_machine_window(
        Position(MACHINE_BUTTON_1),
        Machine::OneToOneCrafter {
            input: Item::Iron,
            output: Item::Wrench,
            cost: 100,
        },
        commands.reborrow(),
        &asset_server,
    );

    commands.spawn((
        SpriteView::OutputPipe,
        OutputPipe {
            item: Item::Wrench,
            reward: 10,
        },
        Position(OUTPUT_PIPE_POS1),
    ));

    commands.spawn((
        SpriteView::BackgroundCity,
        Position(Vec2 { x: 0.0, y: 0.0 }),
    ));
}

pub fn load_level_5(
    mut commands: Commands,
    mut money: ResMut<CurrentMoney>,
    mut money_goal: ResMut<MoneyGoal>,
    asset_server: Res<AssetServer>,
) {
    money.0 = 5000;
    money_goal.0 = 5500;

    setup_mouse_button(commands.reborrow());
    setup_draw_button(commands.reborrow());

    commands.spawn((
        SpriteView::InputPipe,
        Position(INPUT_PIPE_POS2),
        Clickable,
        InputPipe {
            item: Item::Gold,
            spawn_rate: 3.,
            time_elapsed: 0.,
            enabled: false,
            cost: 200,
        },
    ));

    spawn_machine_window(
        Position(MACHINE_BUTTON_1),
        Machine::OneToOneCrafter {
            input: Item::Gold,
            output: Item::Jewelry,
            cost: 200,
        },
        commands.reborrow(),
        &asset_server,
    );

    commands.spawn((
        SpriteView::OutputPipe,
        OutputPipe {
            item: Item::Jewelry,
            reward: 250,
        },
        Position(OUTPUT_PIPE_POS3),
    ));

    commands.spawn((
        SpriteView::BackgroundCity2,
        Position(Vec2 { x: 0.0, y: 0.0 }),
    ));
}

pub fn load_level_6(
    mut commands: Commands,
    mut money: ResMut<CurrentMoney>,
    mut money_goal: ResMut<MoneyGoal>,
    asset_server: Res<AssetServer>,
) {
    money.0 = 1000;
    money_goal.0 = 2000;

    setup_mouse_button(commands.reborrow());
    setup_draw_button(commands.reborrow());

    commands.spawn((
        SpriteView::InputPipe,
        Position(INPUT_PIPE_POS3),
        Clickable,
        InputPipe {
            item: Item::Crystal,
            spawn_rate: 30.,
            time_elapsed: 0.,
            enabled: false,
            cost: 10,
        },
    ));

    spawn_machine_window(
        Position(MACHINE_BUTTON_1),
        Machine::Duplicator { cost: 200 },
        commands.reborrow(),
        &asset_server,
    );

    commands.spawn((
        SpriteView::OutputPipe,
        OutputPipe {
            item: Item::Crystal,
            reward: 9,
        },
        Position(OUTPUT_PIPE_POS3),
    ));

    commands.spawn((
        SpriteView::BackgroundIndustry,
        Position(Vec2 { x: 0.0, y: 0.0 }),
    ));
}

pub fn load_level_7(
    mut commands: Commands,
    mut money: ResMut<CurrentMoney>,
    mut money_goal: ResMut<MoneyGoal>,
    asset_server: Res<AssetServer>,
) {
    money.0 = 1000;
    money_goal.0 = 1001;

    setup_mouse_button(commands.reborrow());
    setup_draw_button(commands.reborrow());

    commands.spawn((
        SpriteView::InputPipe,
        Position(INPUT_PIPE_POS3),
        Clickable,
        InputPipe {
            item: Item::Jewelry,
            spawn_rate: 50.,
            time_elapsed: 0.,
            enabled: false,
            cost: 10,
        },
    ));

    spawn_machine_window(
        Position(MACHINE_BUTTON_1),
        Machine::Duplicator { cost: 200 },
        commands.reborrow(),
        &asset_server,
    );

    commands.spawn((
        SpriteView::OutputPipe,
        OutputPipe {
            item: Item::Jewelry,
            reward: 3,
        },
        Position(OUTPUT_PIPE_POS3),
    ));

    commands.spawn((
        SpriteView::BackgroundCity2,
        Position(Vec2 { x: 0.0, y: 0.0 }),
    ));
}

pub fn load_level_8(
    mut commands: Commands,
    mut money: ResMut<CurrentMoney>,
    mut money_goal: ResMut<MoneyGoal>,
    asset_server: Res<AssetServer>,
) {
    money.0 = 2000;
    money_goal.0 = 4000;

    setup_mouse_button(commands.reborrow());
    setup_draw_button(commands.reborrow());

    commands.spawn((
        SpriteView::InputPipe,
        Position(INPUT_PIPE_POS3),
        Clickable,
        InputPipe {
            item: Item::Iron,
            spawn_rate: 25.,
            time_elapsed: 0.,
            enabled: false,
            cost: 20,
        },
    ));

    spawn_machine_window(
        Position(MACHINE_BUTTON_1),
        Machine::Duplicator { cost: 200 },
        commands.reborrow(),
        &asset_server,
    );

    spawn_machine_window(
        Position(MACHINE_BUTTON_2),
        Machine::OneToOneCrafter {
            input: Item::Iron,
            output: Item::Wrench,
            cost: 100,
        },
        commands.reborrow(),
        &asset_server,
    );

    commands.spawn((
        SpriteView::OutputPipe,
        OutputPipe {
            item: Item::Wrench,
            reward: 15,
        },
        Position(OUTPUT_PIPE_POS3),
    ));

    commands.spawn((
        SpriteView::BackgroundIndustry2,
        Position(Vec2 { x: 0.0, y: 0.0 }),
    ));
}

pub fn load_level_9(
    mut commands: Commands,
    mut money: ResMut<CurrentMoney>,
    mut money_goal: ResMut<MoneyGoal>,
    asset_server: Res<AssetServer>,
) {
    money.0 = 2000;
    money_goal.0 = 190;

    setup_mouse_button(commands.reborrow());
    setup_draw_button(commands.reborrow());

    commands.spawn((
        SpriteView::InputPipe,
        Position(INPUT_PIPE_POS3),
        Clickable,
        InputPipe {
            item: Item::Gold,
            spawn_rate: 3.,
            time_elapsed: 0.,
            enabled: false,
            cost: 200,
        },
    ));

    spawn_machine_window(
        Position(MACHINE_BUTTON_1),
        Machine::Duplicator { cost: 200 },
        commands.reborrow(),
        &asset_server,
    );

    spawn_machine_window(
        Position(MACHINE_BUTTON_2),
        Machine::OneToOneCrafter {
            input: Item::Gold,
            output: Item::Jewelry,
            cost: 100,
        },
        commands.reborrow(),
        &asset_server,
    );

    commands.spawn((
        SpriteView::OutputPipe,
        OutputPipe {
            item: Item::Jewelry,
            reward: 55,
        },
        Position(OUTPUT_PIPE_POS3),
    ));

    commands.spawn((
        SpriteView::BackgroundIndustry2,
        Position(Vec2 { x: 0.0, y: 0.0 }),
    ));
}
