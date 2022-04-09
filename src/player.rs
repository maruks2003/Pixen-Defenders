//! Everything associated with the player.

use bevy::prelude::*;
use crate::texture_handle::*;


/// The plugin of this module -- systems, entities, components, resources....
pub struct PlayerPlugin;

#[derive(Component)]
/// The player component
pub struct Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement);
    }
}

/// Spawn the player entity
fn spawn_player(mut commands: Commands, texture_atlas: Res<PlaceholderPlayer>) {
    // XXX: Fix these
    //   * Sprite index is fixed
    //   * Sprite color is fixed
    //   * Transformation is fixed (not based on resolution)
    //   * The `z` coordinate should be >0.0 so that the sprite doesn't overlap
    //     with others -- the player entity is the most important

    // Get the player sprite
    let mut sprite = TextureAtlasSprite::new(0);
    sprite.color       = Color::rgb(1., 1., 1.);
    sprite.custom_size = Some(Vec2::splat(100.));

    // Spawn the player entity
    let player =
        commands.spawn_bundle(SpriteSheetBundle {
            sprite,
            texture_atlas: texture_atlas.0.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 0.,),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Player"))
        .insert(Player)
        .id();

    // Push it into commands
    commands.entity(player);
}

/// System that handle's the player's movement
fn player_movement(
    mut player: Query<(&Player, &mut Transform)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    // Get the player entity and its `transform`
    let (_, mut transform) = player.single_mut();

    // XXX:
    //   * Input settings are fixed -> WASD
    //   * Movement is fixed and not based on anything
    let movement = 100. * time.delta_seconds();

    // Up
    if input.pressed(KeyCode::W) { transform.translation.y += movement; }
    // Down
    if input.pressed(KeyCode::S) { transform.translation.y -= movement; }
    // Left
    if input.pressed(KeyCode::A) { transform.translation.x -= movement; }
    // Right
    if input.pressed(KeyCode::D) { transform.translation.x += movement; }
}
