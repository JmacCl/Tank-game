use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};
use amethyst::ecs::world::Generation;

pub struct Menu;

// Constants for arena variables size
pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

impl SimpleState for Menu{
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // Get data of world
        let world = data.world;
        // Set up the camera for the world
        initialise_camera(world);
    }

}

fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.

    // Below sets up the component to represent the position in the world
    let mut transform = Transform::default();
    // Set the position to be the middle of the screen, at distance 1 from xy plane
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    // Build into the world with entity with the camera, has width and height as defined in parameters
    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

