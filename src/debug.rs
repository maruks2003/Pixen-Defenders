//! Bevy inspector egui plugin

use bevy::{
    prelude::*,
    input::system::exit_on_esc_system,
};
use bevy_inspector_egui::{
    WorldInspectorPlugin,
    RegisterInspectable,
};
use crate::{
    player::Player,
    attributes::*,
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new())
                .register_inspectable::<Player>()
                .register_inspectable::<MovementSpeed>()
                .add_system(exit_on_esc_system);
        }
    }
}
