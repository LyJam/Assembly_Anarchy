use crate::game::*;

use bevy::sprite::Anchor;

#[derive(Component)]
pub enum SpriteView {
    InputPipe,
    Item { item: Item },
}

impl SpriteView {
    pub fn get_sprite(&self) -> &'static str {
        match self {
            SpriteView::InputPipe => "input_pipe.png",
            SpriteView::Item { item } => item.get_sprite(),
        }
    }

    pub fn get_name(&self) -> &'static str {
        match self {
            SpriteView::InputPipe => "pipe",
            SpriteView::Item { item } => item.get_name(),
        }
    }

    pub fn get_scale(&self) -> Vec2 {
        match self {
            SpriteView::InputPipe => Vec2::new(200.0, 200.0),
            SpriteView::Item { item: _ } => Vec2::new(32.0, 32.0),
        }
    }

    pub fn get_layer(&self) -> f32 {
        match self {
            SpriteView::InputPipe => 10.,
            SpriteView::Item { item: _ } => 0.,
        }
    }

    pub fn get_anchor(&self) -> Anchor {
        match self {
            _ => Anchor::Center,
        }
    }
}

pub fn on_add_view(
    trigger: Trigger<OnAdd, SpriteView>,
    asset_server: Res<AssetServer>,
    views: Query<(&SpriteView, &Position)>,
    mut commands: Commands,
) {
    let entity = trigger.entity();
    if let Ok((view, pos)) = views.get(entity) {
        let mut sprite = Sprite::from_image(asset_server.load(view.get_sprite()));
        sprite.custom_size = Some(view.get_scale());
        sprite.anchor = view.get_anchor();
        commands.entity(entity).insert((
            sprite,
            Transform::from_xyz(pos.0.x, pos.0.y, view.get_layer()),
        ));
    }
}
