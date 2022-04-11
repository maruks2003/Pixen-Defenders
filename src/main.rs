use bevy::{
    prelude::*,
};
use pixen_defenders::{
    constants::*,
    texture_handle::*,
    player::*,
    tiles::*,
    debug::*,
    settings::*,
};



fn main() {
    App::new()
        // Set up the game. This has to be done `PreStartup` because we have to
        // load the resources before other plugins start using them
        .add_startup_system_to_stage(StartupStage::PreStartup, setup)

        // Spawn the player
        .add_plugin(PlayerPlugin)

        // Spawn the tiles
        .add_plugin(TilesPlugin)

        // Default plugins..
        .add_plugins(DefaultPlugins)

        // Debug egui
        .add_plugin(DebugPlugin)

        // Run the game
        .run();
}

/// Game setup -- spawn the camera, load the assets... :^)
fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>
 ) {
    // Set the clear background color
    commands.insert_resource(ClearColor(CLEAR_COLOR));

    // Spawn the settings
    let settings = GameSettings::new();

    // Set up the window
    commands.insert_resource(WindowDescriptor {
        width:     settings.screen_width,
        height:    settings.screen_height,
        title:     GAME_TITLE.to_string(),
        vsync:     settings.use_vsync,
        resizable: false,
        ..Default::default()
    });

    // Spawn the camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Load the CP437 asset, parse it and insert it into commands as a resource
    let cp437 = assets.load("code_page.png");
    let atlas = TextureAtlas::from_grid_with_padding(
        cp437, Vec2::splat(9.), 16, 16, Vec2::splat(2.)
    );
    commands.insert_resource(Cp437(atlases.add(atlas)));

    // Load the temporary wall tileset
    let wall_tileset = assets.load("pixen_defenders.png");
    let atlas = TextureAtlas::from_grid(
        wall_tileset, Vec2::splat(16.), 3, 3
    );
    commands.insert_resource(WallTileset(atlases.add(atlas)));

    // Finally, insert the settings into resources
    commands.insert_resource(GameSettings::new());
}
