use bevy::{
    prelude::*,
};
use pixen_defenders::{
    constants::*,
    texture_handle::*,
    player::*,
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

        // Set up the game. This has to be done `PreStartup` because we have to
        // load the resources before other plugins start using them
        .add_startup_system_to_stage(StartupStage::PreStartup, setup)

        // Spawn the player
        .add_plugin(PlayerPlugin)

        // Default plugins..
        .add_plugins(DefaultPlugins)

        // Run the game
        .run();
}

/// Game setup -- spawn the camera, load the assets... :^)
fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>
 ) {
    // Spawn the camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Load the CP437 asset, parse it and insert it into commands as a resource
    let cp437 = assets.load("code_page.png");
    let atlas = TextureAtlas::from_grid_with_padding(
        cp437, Vec2::splat(9.), 16, 16, Vec2::splat(2.)
    );
    commands.insert_resource(Cp437(atlases.add(atlas)));


    // Load the placeholder player sprite, parse it, make it a resource...
    // TODO: Maybe there's a better way to load a single sprite?
    let player_sprite = assets.load("placeholder.png");
    let atlas = TextureAtlas::from_grid(
        player_sprite, Vec2::splat(32.), 1, 1
    );
    commands.insert_resource(PlaceholderPlayer(atlases.add(atlas)));
}
