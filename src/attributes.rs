//! Various attributes (in the form of components) used in the world
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Component, Inspectable)]
/// The component associated with entities that can move at a certain speed
pub struct MovementSpeed {
    /// Movement vector of the component
    pub mov_vec: Vec2,
}

impl MovementSpeed {
    /// Creates a new `MovementSpeed` component from `x` and `y`
    pub fn new(x: f32, y: f32) -> Self {
        Self { mov_vec: Vec2::new(x, y), }
    }

    /// Creates a new `MovementSpeed` component from an already defined vector
    pub fn from_vec(vec: Vec2) -> Self {
        Self { mov_vec: vec, }
    }
}

// When dereferencing the component, get the inner vector instead of
// the component itself.
impl std::ops::Deref for MovementSpeed {
    type Target = Vec2;
    fn deref(&self) -> &Self::Target {
        &self.mov_vec
    }
}
