//! Everything associated with the player.

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use crate::{
    texture_handle::*,
    attributes::*,
};


/// The plugin of this module -- systems, entities, components, resources....
pub struct PlayerPlugin;

#[derive(Component, Inspectable)]
/// The player component
pub struct Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement);
    }
}

/// Spawn the player entity
fn spawn_player(mut commands: Commands, texture_atlas: Res<Cp437>) {
    // XXX: Fix these
    //   * Sprite index is fixed
    //   * Sprite color is fixed
    //   * Transformation is fixed (not based on resolution)
    //   * The `z` coordinate should be >0.0 so that the sprite doesn't overlap
    //     with others -- the player entity is the most important

    // Get the player sprite
    let mut sprite = TextureAtlasSprite::new(2);
    sprite.color       = Color::rgb_u8(150, 70, 170);
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
        .insert(MovementSpeed::from_vec(Vec2::splat(100.)))
        .id();

    // Push it into commands
    commands.entity(player);
}

/// System that handle's the player's movement
fn player_movement(
    mut player: Query<(&Player, &mut Transform, &MovementSpeed)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    // Get the player entity and its `transform`
    let (_, mut transform, speed) = player.single_mut();

    // XXX:
    //   * Input settings are fixed -> WASD
    let movement_x = speed.x * time.delta_seconds();
    let movement_y = speed.y * time.delta_seconds();

    // Up
    if input.pressed(KeyCode::W) { transform.translation.y += movement_y; }
    // Down
    if input.pressed(KeyCode::S) { transform.translation.y -= movement_y; }
    // Left
    if input.pressed(KeyCode::A) { transform.translation.x -= movement_x; }
    // Right
    if input.pressed(KeyCode::D) { transform.translation.x += movement_x; }
}
