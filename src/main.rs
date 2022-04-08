use bevy::{prelude::*, render::camera::ScalingMode};

mod player;
mod debug;

use player::PlayerPlugin;
use debug::DebugPlugin;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.1;

fn main() {
    let height = 900.0;
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: height*RESOLUTION,
            height: height,
            title: "Pixen defenders".to_string(),
            vsync: true,
            resizable: false,

            ..Default::default()
        })

        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_plugin(PlayerPlugin)
        .add_plugin(DebugPlugin)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_assets)
        .run();

}



fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}

struct AsciiSheet(Handle<TextureAtlas>);

fn load_assets(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
    ){
    let image = assets.load("placeholder.png");
    let atlas = TextureAtlas::from_grid_with_padding(
        image,
        Vec2::splat(TILE_SIZE),
        1,
        1,
        Vec2::splat(0.)
        );

    let atlas_handle = texture_atlases.add(atlas);

    commands.insert_resource(AsciiSheet(atlas_handle));
}
