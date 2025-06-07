use crate::game::*;
use bevy::window::{PrimaryWindow, Window};

#[derive(Resource)]
pub struct MouseWorldPosition(pub Option<Position>);

#[derive(Resource)]
pub struct LeftMouseClickPosition(pub Option<Position>);

#[derive(Component)]
pub struct Clickable;

#[derive(Resource)]
pub struct JustClicked(pub Option<Entity>);

pub fn update_mouse_world_position(
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut mouse_world_pos: ResMut<MouseWorldPosition>,
) {
    let window = windows.single();
    let (camera, cam_transform) = cameras.single();

    if let Some(cursor_pos) = window.cursor_position() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(cam_transform, cursor_pos) {
            mouse_world_pos.0 = Some(Position(world_pos)); // Truncate to Vec2
        } else {
            mouse_world_pos.0 = None; // Handle case where conversion fails
        }
    } else {
        mouse_world_pos.0 = None; // No cursor position
    }
}

pub fn update_left_mouse_click_position(
    buttons: Res<ButtonInput<MouseButton>>,
    mut left_mouse_click_pos: ResMut<LeftMouseClickPosition>,
    mouse_world_pos: ResMut<MouseWorldPosition>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        left_mouse_click_pos.0 = mouse_world_pos.0;
    } else {
        left_mouse_click_pos.0 = None; // Reset if not clicked
    }
}

pub fn update_just_clicked(
    left_mouse_click_pos: Res<LeftMouseClickPosition>,
    mut just_clicked: ResMut<JustClicked>,
    views: Query<(Entity, &SpriteView, &Position), With<Clickable>>,
) {
    just_clicked.0 = None;

    // If there was no left mouse click this frame, we have nothing to do.
    let Some(click_world_pos) = left_mouse_click_pos.0 else {
        return;
    };

    // Iterate over all entities that are `Clickable` and have a `SpriteView` and `Position`.
    for (entity, sprite_view, entity_pos) in views.iter() {
        // Get the scale/size of the sprite from its SpriteView
        let sprite_size = sprite_view.get_scale();
        let half_size = sprite_size / 2.0;

        // Calculate the bounding box for the sprite in world coordinates.
        // Assuming Anchor::Center means the Position is the center of the sprite.
        let min_x = entity_pos.0.x - half_size.x;
        let max_x = entity_pos.0.x + half_size.x;
        let min_y = entity_pos.0.y - half_size.y;
        let max_y = entity_pos.0.y + half_size.y;

        // Create a Bevy Rect for easy collision checking.
        let bounding_box = Rect::new(min_x, min_y, max_x, max_y);

        // Check if the click position is within the bounding box.
        if bounding_box.contains(click_world_pos.0) {
            just_clicked.0 = Some(entity);
            info!("Entity {:?} was just clicked!", entity);
            return; // Exit the function as we found the clicked entity.
        }
    }
}
