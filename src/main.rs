use bevy::{
    prelude::*,
};
use pixen_defenders::{
    constants::*,
};


fn main() {
    App::new()
        // Set the clear background color
        .insert_resource(ClearColor(CLEAR_COLOR))

        // Window settings
        // XXX: Fix these
        //   * Fixed resolution
        //   * Vsync always on
        // TODO: Add some more settings
        .insert_resource(WindowDescriptor {
            width:     1920.,
            height:    1080.,
            title:     GAME_TITLE.to_string(),
            vsync:     true,
            resizable: false,
            ..Default::default()
        })

        .add_plugins(DefaultPlugins)
        .run();
}
