use bevy::{
    prelude::*,
};
use pixen_defenders::{
    constants::*,
};


fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR_COLOR))
        .add_plugins(DefaultPlugins)
        .run();
}
