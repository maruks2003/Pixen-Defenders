//! In game settings that can be changed by the user
use bevy_inspector_egui::Inspectable;

/// The struct that can be accessed as a resource from the global memory space
#[derive(Inspectable)]
pub struct GameSettings {
    /// Width of the window (in px)
    pub screen_width: f32,

    /// Height of the window (in px)
    pub screen_height: f32,

    /// Size of each tile. This is variable and can change based
    /// on the zoom factor
    pub tile_size: f32,

    /// Whether we should use vsync or not
    pub use_vsync: bool,
}

impl GameSettings {
    /// Creates a new struct with the default settings
    pub fn new() -> Self {
        Self { ..Default::default() }
    }
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            screen_width: 1920.,
            screen_height: 1080.,
            tile_size: 75.,
            use_vsync: false,
        }
    }
}
