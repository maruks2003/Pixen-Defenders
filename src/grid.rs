use bevy::{
    prelude::*,
    ecs::system::EntityCommands,
};
use crate::{
    AsciiSheet,
    TILE_SIZE,
};


pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_grid);
    }
}

enum TileType {
    Floor,
    Wall,
}

// A struct which holds a grid of entities, used as the game map
struct GridMap {
    map: Vec<Vec<Entity>>,
    position: Vec2,
}

impl GridMap {

    // Creates the grid of desired size at desired position(top left corner)
    // and fills it with the requested entity
    // (with no regard to changing the transform so fuck me)
    //
    // TODO make it so all the tiles arent fucking placed in one position
    fn new(width: usize, height: usize,
           position: Vec2, fill_entity: TileType) -> Self {
        let m = vec![vec![fill_entity; height]; width];
        for y in 0..height {
            for x in 0..width {
                m[x][y] = m[x][y].insert(Transform {
                    translation: Vec3::new(position.x+TILE_SIZE*x,
                                           position.y+TILE_SIZE*y, 800.0),
                    ..Default::default()
                })
            }
        }

        GridMap {
            map: m,
            position: position,
        }
    }

    fn insert_entity(&mut self, x: usize, y: usize, e: Entity) {
        self.map[y][x] = e;
    }
}

fn spawn_grid(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let mut sprite = TextureAtlasSprite::new(0);
    let mut tile_template = commands.spawn()
        .insert(sprite)
        .insert(ascii.0.clone())
        .insert(Name::new("Default Tile"));

    let grid = GridMap::new(8,8, Vec2::new(0.0, 0.0), TileType::Floor);

    for y in grid.map {
        for e in y{
            print!("");
        }
    }
}
