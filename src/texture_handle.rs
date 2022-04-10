//! Handles to various texture atlases
use bevy::prelude::*;

/// Code Page 437
pub struct Cp437(pub Handle<TextureAtlas>);

/// Wall tileset
pub struct WallTileset(pub Handle<TextureAtlas>);
