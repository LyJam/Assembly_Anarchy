use crate::game::*;
use bevy::color::palettes::basic::*;

#[derive(Component)]
pub struct DrawIndicator;

pub fn draw_obstacle(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    mouse_pos: Res<MouseWorldPosition>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    over_element: Res<OverClickableElement>,
    dragging: Res<Dragging>,
    tool_selected: Res<SelectedTool>,
    mut draw_indicator: Query<(&mut Transform), With<DrawIndicator>>,
) {
    if (tool_selected.0 == Tools::Draw) {
        if let Some(_) = over_element.0 {
            // move indicator off screen
            if let mut transform = draw_indicator.single_mut() {
                *transform = Transform::from_translation(Vec3 {
                    x: 10000.,
                    y: 10000.,
                    z: -1000.,
                })
                .with_scale(Vec3::splat(24.));
            }
            // don't draw over non-clickable elements
            return;
        }
        if (dragging.entity != None) {
            // move indicator off screen
            if let mut transform = draw_indicator.single_mut() {
                *transform = Transform::from_translation(Vec3 {
                    x: 10000.,
                    y: 10000.,
                    z: -1000.,
                })
                .with_scale(Vec3::splat(24.));
            }
            // don't draw over non-clickable elements
            return;
        }
        // move indicator to mouse position
        if let Some(mouse_position) = mouse_pos.0 {
            if let mut transform = draw_indicator.single_mut() {
                *transform = Transform::from_translation(Vec3 {
                    x: mouse_position.0.x,
                    y: mouse_position.0.y,
                    z: 90.,
                })
                .with_scale(Vec3::splat(24.));
            }

            //draw
            if (buttons.pressed(MouseButton::Left)) {
                commands.spawn((
                    Mesh2d(meshes.add(Circle::default())),
                    MeshMaterial2d(materials.add(Color::from(BLACK))),
                    Transform::from_xyz(mouse_position.0.x, mouse_position.0.y, 1.)
                        .with_scale(Vec3::splat(24.)),
                    Position(Vec2 {
                        x: mouse_position.0.x,
                        y: mouse_position.0.y,
                    }),
                    CirclePhysics { radius: 12. },
                ));
            }
        }
    } else {
        // move indicator off screen
        if let mut transform = draw_indicator.single_mut() {
            *transform = Transform::from_translation(Vec3 {
                x: 10000.,
                y: 10000.,
                z: -1000.,
            })
            .with_scale(Vec3::splat(24.));
        }
    }
}
