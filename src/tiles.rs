use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use crate::{
    texture_handle::*,
    settings::*,
};

/// The plugin of this module
pub struct TilesPlugin;

#[derive(Component, Inspectable)]
/// The tile component
pub struct Tile;

impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_tiles);
    }
}

fn spawn_tiles(
    mut commands: Commands,
    texture_atlas: Res<WallTileset>,
    settings: Res<GameSettings>,
) {
    // Get the tile sprite
    // TODO system for resolving which sprite to use
    let mut sprite = TextureAtlasSprite::new(0);
    sprite.custom_size = Some(Vec2::splat(settings.tile_size));

    // spawn the tile entity
    let tile =
        commands.spawn_bundle(SpriteSheetBundle {
            sprite,
            texture_atlas: texture_atlas.0.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 2.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Testing tile"))
        .insert(Tile)
        .id();
}
