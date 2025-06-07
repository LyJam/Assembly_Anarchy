use crate::game::*;
use bevy::utils::HashMap;

// A resource to track the current level number.
#[derive(Resource, Default)]
pub struct CurrentLevel(pub u32);

// A resource that holds a map of level numbers to their spawn functions.
#[derive(Resource)]
pub struct LevelRegistry(pub HashMap<u32, fn(Commands) -> ()>); // A map from level number to a spawn function

impl Default for LevelRegistry {
    fn default() -> Self {
        let mut map = HashMap::new();
        // Register your level spawn functions here:
        map.insert(0u32, load_level_1 as fn(Commands));
        map.insert(1u32, load_level_1 as fn(Commands));
        LevelRegistry(map)
    }
}

/// Event triggered when the current level is completed.
#[derive(Event)]
pub struct LevelCompleted;
pub fn load_initial_level(
    mut commands: Commands,
    level_registry: Res<LevelRegistry>,
    current_level: Res<CurrentLevel>,
) {
    if let Some(spawn_fn) = level_registry.0.get(&current_level.0) {
        spawn_fn(commands);
    } else {
        error!(
            "No spawn function registered for initial level {}",
            current_level.0
        );
    }
}

pub fn load_next_level(
    mut commands: Commands,
    mut level_completed_events: EventReader<LevelCompleted>,
    mut current_level: ResMut<CurrentLevel>,
    level_registry: Res<LevelRegistry>,
    level_entities_query: Query<Entity, With<Position>>, // Query needed for de-spawn
) {
    // Only proceed if a LevelCompleted event occurred
    if level_completed_events.is_empty() {
        return;
    }

    // Clear the events so they don't trigger again next frame
    level_completed_events.clear();

    info!("Level completed! Loading next level...");

    // Despawn all entities from the current level
    for entity in level_entities_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    current_level.0 += 1; // Increment the level

    // Check if the next level exists in the registry
    if let Some(spawn_fn) = level_registry.0.get(&current_level.0) {
        info!("Loading Level: {}", current_level.0);
        spawn_fn(commands); // Call the spawn function for the new level
    } else {
        // No more levels defined, handle "Game Over" or loop back to level 0
        info!(
            "No more levels found. Current level: {}. Game Over!",
            current_level.0
        );
    }
}

// TODO: remove before publishing
pub fn simulate_level_completion(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut level_completed_writer: EventWriter<LevelCompleted>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        info!("Simulating level completion (Spacebar pressed).");
        level_completed_writer.send(LevelCompleted);
    }
}

pub fn load_level_1(mut commands: Commands) {
    commands.spawn((
        SpriteView::InputPipe,
        Position(INPUT_PIPE_POS1),
        Clickable,
        InputPipe {
            item: Item::Bolt,
            spawn_rate: 5.,
            time_elapsed: 0.,
            enabled: false,
        },
    ));
    commands.spawn((SpriteView::InputPipe, Position(INPUT_PIPE_POS2), Clickable));
    commands.spawn((SpriteView::InputPipe, Position(INPUT_PIPE_POS3), Clickable));
    commands.spawn((SpriteView::InputPipe, Position(INPUT_PIPE_POS4), Clickable));
    commands.spawn((SpriteView::InputPipe, Position(INPUT_PIPE_POS5), Clickable));

    commands.spawn((
        SpriteView::OutputPipe,
        OutputPipe { item: Item::Bolt },
        Position(OUTPUT_PIPE_POS3),
    ));

    commands.spawn((
        SpriteView::BackgroundIndustry,
        Position(Vec2 { x: 0.0, y: 0.0 }),
    ));
}
