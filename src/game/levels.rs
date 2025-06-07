use crate::game::*;

pub fn load_level(mut commands: Commands) {
    commands.spawn((
        SpriteView::InputPipe,
        Position(INPUT_PIPE_POS1),
        Clickable,
        InputPipe {
            item: Item::Bolt,
            spawn_rate: 5.,
            time_elapsed: 0.,
        },
    ));
    commands.spawn((SpriteView::InputPipe, Position(INPUT_PIPE_POS2), Clickable));
    commands.spawn((SpriteView::InputPipe, Position(INPUT_PIPE_POS3), Clickable));
    commands.spawn((SpriteView::InputPipe, Position(INPUT_PIPE_POS4), Clickable));
    commands.spawn((SpriteView::InputPipe, Position(INPUT_PIPE_POS5), Clickable));
}
