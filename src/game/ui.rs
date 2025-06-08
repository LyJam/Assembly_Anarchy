use bevy::sprite::Anchor;

use crate::game::*;
use bevy::winit::cursor::CursorIcon;
use bevy::{
    state::commands,
    window::{PrimaryWindow, SystemCursorIcon, Window},
};

#[derive(Component)]
pub struct LevelText;

#[derive(Component)]
pub struct MoneyText;

#[derive(Component)]
pub struct GoalText;

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // spawn money display
    let mut sprite = Sprite::from_image(asset_server.load("UI/money_window.png"));
    sprite.custom_size = Some(Vec2 { x: 320., y: 192. });
    sprite.anchor = Anchor::TopLeft;
    commands.spawn((sprite, Transform::from_xyz(-800., 450., 100.)));

    commands.spawn((
        Text::new("Level: ./."),
        TextFont {
            font: asset_server.load("Fonts/CyberpunkCraftpixPixel.otf"),
            font_size: 20.,
            ..default()
        },
        TextColor(Color::srgb(211.0 / 255.0, 211.0 / 255.0, 211.0 / 255.0)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(25.0),
            left: Val::Px(30.0),
            ..default()
        },
        LevelText,
    ));

    commands.spawn((
        Text::new("10000"),
        TextFont {
            font: asset_server.load("Fonts/CyberpunkCraftpixPixel.otf"),
            font_size: 40.,
            ..default()
        },
        TextColor(Color::srgb(255.0 / 255.0, 215.0 / 255.0, 0.0)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(60.0),
            left: Val::Px(100.0),
            ..default()
        },
        MoneyText,
    ));

    commands.spawn((
        Text::new("Goal: 100000"),
        TextFont {
            font: asset_server.load("Fonts/CyberpunkCraftpixPixel.otf"),
            font_size: 20.,
            ..default()
        },
        TextColor(Color::srgb(211.0 / 255.0, 211.0 / 255.0, 211.0 / 255.0)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(110.0),
            left: Val::Px(30.0),
            ..default()
        },
        GoalText,
    ));
}

pub fn update_level_text(
    current_level: Res<CurrentLevel>,
    level_list: Res<LevelRegistry>,
    mut level_text: Query<&mut Text, With<LevelText>>,
) {
    let total_levels = level_list.0.iter().count();
    level_text.single_mut().0 = format!("Level: {:?}/{:?}", current_level.0, total_levels);
}

pub fn update_money_text(
    current_money: Res<CurrentMoney>,
    mut money_text: Query<&mut Text, With<MoneyText>>,
) {
    money_text.single_mut().0 = format!("{:?}", current_money.0);
}

pub fn update_goal_text(
    current_goal: Res<MoneyGoal>,
    mut goal_text: Query<&mut Text, With<GoalText>>,
) {
    goal_text.single_mut().0 = format!("Goal: {:?}", current_goal.0);
}

pub fn update_mouse_pointer(
    over_element: Res<OverClickableElement>,
    mut window: Query<Entity, With<PrimaryWindow>>,
    mut commands: Commands,
) {
    if let primary_window = window.single_mut() {
        if let Some(_) = over_element.0 {
            commands
                .entity(primary_window)
                .insert((CursorIcon::System(SystemCursorIcon::Pointer)));
        } else {
            commands
                .entity(primary_window)
                .insert((CursorIcon::System(SystemCursorIcon::default())));
        }
    }
}
