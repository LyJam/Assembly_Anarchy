use crate::game::*;

use bevy::sprite::Anchor;

#[derive(Component)]
pub enum SpriteView {
    InputPipe,
    OutputPipe,
    CursorPointLeft,
    BackgroundIndustry,
    BackgroundIndustry2,
    BackgroundCity,
    Item { item: Item, ui_element: bool },
}

impl SpriteView {
    pub fn get_sprite(&self) -> &'static str {
        match self {
            SpriteView::InputPipe => "input_pipe.png",
            SpriteView::OutputPipe => "output_pipe.png",
            SpriteView::BackgroundIndustry => "Backgrounds/industry.png",
            SpriteView::BackgroundIndustry2 => "Backgrounds/industry2.png",
            SpriteView::BackgroundCity => "Backgrounds/city.png",
            SpriteView::CursorPointLeft => "UI/cursor_point_left.png",
            SpriteView::Item {
                item,
                ui_element: _,
            } => item.get_sprite(),
        }
    }

    pub fn get_name(&self) -> &'static str {
        match self {
            SpriteView::InputPipe => "pipe",
            SpriteView::OutputPipe => "pipe",
            SpriteView::CursorPointLeft => "pointer",
            SpriteView::BackgroundIndustry => "Background",
            SpriteView::BackgroundIndustry2 => "Background",
            SpriteView::BackgroundCity => "Background",
            SpriteView::Item {
                item,
                ui_element: _,
            } => item.get_name(),
        }
    }

    pub fn get_scale(&self) -> Vec2 {
        match self {
            SpriteView::InputPipe => Vec2::new(150.0, 100.0),
            SpriteView::OutputPipe => Vec2::new(200.0, 200.0),
            SpriteView::CursorPointLeft => Vec2::new(100.0, 100.0),
            SpriteView::BackgroundIndustry => Vec2::new(1600.0, 900.0),
            SpriteView::BackgroundIndustry2 => Vec2::new(1600.0, 900.0),
            SpriteView::BackgroundCity => Vec2::new(1600.0, 900.0),
            SpriteView::Item {
                item: _,
                ui_element,
            } => {
                if (*ui_element) {
                    Vec2::new(64.0, 64.0)
                } else {
                    Vec2::new(32.0, 32.0)
                }
            }
        }
    }

    pub fn get_layer(&self) -> f32 {
        match self {
            SpriteView::InputPipe => 10.,
            SpriteView::OutputPipe => 10.,
            SpriteView::CursorPointLeft => 10.,
            SpriteView::BackgroundIndustry => -10.,
            SpriteView::BackgroundIndustry2 => -10.,
            SpriteView::BackgroundCity => -10.,
            SpriteView::Item {
                item: _,
                ui_element,
            } => {
                if (*ui_element) {
                    100.
                } else {
                    0.
                }
            }
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

/* remove items that fall out of the game space (for performance) */
pub fn remove_escaped_items(mut commands: Commands, items: Query<(Entity, &Position)>) {
    for (entity, pos) in items.iter() {
        if (pos.0.distance(Vec2 { x: 0., y: 0. }) > 1000.0) {
            commands.entity(entity).despawn();
        }
    }
}
