use bevy::color::palettes::basic::*;
use bevy::sprite::Anchor;
use bevy::transform;

use crate::game::*;
use bevy::winit::cursor::CursorIcon;
use bevy::{
    state::commands,
    window::{PrimaryWindow, SystemCursorIcon, Window},
};

pub const BUTTON_MOUSE_POS: Vec2 = Vec2::new(-740., 200.);
pub const BUTTON_DRAW_POS: Vec2 = Vec2::new(-740., 90.);

#[derive(Resource)]
pub struct SelectedTool(pub Tools);

#[derive(Component, Debug, Copy, Clone, PartialEq)]
pub enum Tools {
    Mouse,
    Draw,
}

#[derive(Component)]
pub struct SelectionIndicator;

#[derive(Component)]
pub struct LevelText;

#[derive(Component)]
pub struct MoneyText;

#[derive(Component)]
pub struct GoalText;

pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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

    // spawn selection indicator
    let mut sprite = Sprite::from_image(asset_server.load(SpriteView::ArrowLeft.get_sprite()));
    sprite.custom_size = Some(SpriteView::ArrowLeft.get_scale());
    commands.spawn((
        SelectionIndicator,
        Transform::from_translation(Vec3 {
            x: 10000.,
            y: 10000.,
            z: -1000.,
        }),
        sprite,
    ));

    // spawn draw indicator
    commands.spawn((
        DrawIndicator,
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(Color::from(BLACK))),
        Transform::from_xyz(10000., 10000., 1.).with_scale(Vec3::splat(24.)),
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

pub fn tool_selection(
    tools: Query<(Entity, &Tools)>,
    just_clicked: Res<JustClicked>,
    mut selected: ResMut<SelectedTool>,
    mut selection_indicator: Query<(&mut Transform), With<SelectionIndicator>>,
) {
    let mut any_ui_ellement = false;
    for (tool_entity, tool) in tools.iter() {
        any_ui_ellement = true;
        if let Some(clicked_entity) = just_clicked.0 {
            if (tool_entity == clicked_entity) {
                selected.0 = *tool;
            }
        }
    }

    // move selection indicator
    if let mut transform = selection_indicator.single_mut() {
        if (!any_ui_ellement) {
            // move selection indicator off-screen
            *transform = Transform::from_translation(Vec3 {
                x: 10000.,
                y: 10000.,
                z: -1000.,
            });
        } else {
            match selected.0 {
                Tools::Mouse => {
                    *transform = Transform::from_translation(Vec3 {
                        x: BUTTON_MOUSE_POS.x + 80.,
                        y: BUTTON_MOUSE_POS.y,
                        z: 101.,
                    })
                }
                Tools::Draw => {
                    *transform = Transform::from_translation(Vec3 {
                        x: BUTTON_DRAW_POS.x + 80.,
                        y: BUTTON_DRAW_POS.y,
                        z: 101.,
                    })
                }
            }
        }
    }
}

pub fn setup_mouse_button(mut commands: Commands) {
    commands.spawn((
        SpriteView::ButtonMouse,
        Position(BUTTON_MOUSE_POS),
        Clickable,
        Tools::Mouse,
    ));
}

pub fn setup_draw_button(mut commands: Commands) {
    commands.spawn((
        SpriteView::ButtonDraw,
        Position(BUTTON_DRAW_POS),
        Clickable,
        Tools::Draw,
    ));
}
