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

        // Start setting up the game
        .add_startup_system(setup)

        // Default plugins..
        .add_plugins(DefaultPlugins)

        // Run the game
        .run();
}


/// An enum differentiating between the various atlas handles in the global
/// resource space.
pub enum TextureHandle {
    Cp437(Handle<TextureAtlas>),
}

/// Game setup -- spawn the camera, load the assets... :^)
fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>
 ) {
    // Spawn the camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Load the CP437 asset and parse it
    let cp437 = assets.load("code_page.png");
    let atlas = TextureAtlas::from_grid_with_padding(
        cp437, Vec2::splat(9.), 16, 16, Vec2::splat(2.)
    );
    // And insert it into `commands` as a resource
    commands.insert_resource(TextureHandle::Cp437(atlases.add(atlas)));
}
