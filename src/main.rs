use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;

mod game;
use game::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics in web builds on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Assembly Anarchy".into(),
                        resolution: (1600., 900.).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(MouseWorldPosition(None))
        .insert_resource(LeftMouseClickPosition(None))
        .insert_resource(JustClicked(None))
        .insert_resource(OverClickableElement(None))
        .insert_resource(LevelRegistry::default())
        .insert_resource(CurrentLevel(0))
        .insert_resource(CurrentMoney(-1))
        .insert_resource(MoneyGoal(0))
        .insert_resource(LevelWon(false))
        .insert_resource(LevelLost(false))
        .add_observer(on_add_view)
        .add_observer(on_add_output_pipe)
        .add_observer(on_add_input_pipe)
        .add_systems(Startup, (setup_camera, setup_ui, load_initial_level))
        .add_systems(
            Update,
            (
                add_gravity,
                collision_with_static_circles,
                collision_with_static_rectangles,
                update_position,
                on_changed_position,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                update_level_text,
                update_money_text,
                update_goal_text,
                update_mouse_pointer,
            ),
        )
        .add_systems(
            Update,
            (
                toggle_input_pipe,
                input_pipe_spawn_item,
                output_pipe_consume_item,
                draw_obstacle,
                remove_escaped_items,
                level_management,
            ),
        )
        .add_systems(
            Update,
            (
                update_mouse_world_position,
                update_left_mouse_click_position,
                update_just_clicked,
            )
                .chain(),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d));
}
