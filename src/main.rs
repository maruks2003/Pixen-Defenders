use macroquad::prelude::*;

#[macroquad::main("Pixen defenders")]
async fn main() {
    loop {
        clear_background(BLACK);
    }
}

enum AttackPriority{
    LeastPhysicalArmour,
    MostPhysicalArmour,
    LeastMagicalArmour,
    MostMagicalArmour,
    Slowest,
    Fastest,
    First,
    Last,
}

struct Tower{
    position: Vec2,
    name: String,
    range: u32,
    phys_damage: u32,
    mag_damage: u32,
    pure_damage: u32,
    attack_speed: f32,
    true_sight: bool, //whether the tower can attack invisible units
    aoe: bool,
    aoe_radius: u32,
    attack_priority: u8, //who will the tower attack first
    texture: Texture2D,
}

struct Enemy{
    position: Vec2,
    hp: u32,
    npc_damage: u32, //the damage the enemy will deal to npcs
    player_damage: u32, //the damage the enemy will deal when he gets through
    invisible: bool,
    attack_range: u32,
    texture: Texture2D,
}
