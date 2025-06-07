use crate::game::*;
use bevy::color::palettes::basic::*;

pub fn draw_obstacle(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    mouse_pos: Res<MouseWorldPosition>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if (buttons.pressed(MouseButton::Left)) {
        if let Some(mouse_position) = mouse_pos.0 {
            commands.spawn((
                Mesh2d(meshes.add(Circle::default())),
                MeshMaterial2d(materials.add(Color::from(BLACK))),
                Transform::from_xyz(mouse_position.0.x, mouse_position.0.y, 1.)
                    .with_scale(Vec3::splat(32.)),
                Position(Vec2 {
                    x: mouse_position.0.x,
                    y: mouse_position.0.y,
                }),
                CirclePhysics { radius: 16. },
            ));
        }
    }
}
