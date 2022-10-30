use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};
use amethyst::ecs::world::Generation;

pub struct Pong;

// Constants for arena variables size
pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

// Constants for the paddles height and widths
pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;

#[derive(PartialEq, Eq)]
pub enum Side{
    Left,
    Right,
}
pub struct Paddle{
    pub side:Side,
    pub width: f32,
    pub height:f32,
}

impl Paddle {
    fn new(side: Side) -> Paddle {
        Paddle {
            side,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
        }
    }
}

impl Component for Paddle{
    // Allows Paddle to be attached to entities in game
    // Defined ths storage for specific optimization and lower memory usage
    type Storage = DenseVecStorage<Self>;
}

/// Initialises one paddle on the left, and one paddle on the right.
fn initialise_paddles(world: &mut World) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    // Position Paddles Correctly:
    let y = ARENA_HEIGHT/ 2.0;
    left_transform.set_translation_xyz(PADDLE_WIDTH * 0.5,y, 0.0 );
    right_transform.set_translation_xyz(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);

    // Create a left plank entity;
    world
        .create_entity()
        .with(Paddle::new(Side::Left))
        .with(left_transform)
        .build();

    // Creating right plank entity
    world
        .create_entity()
        .with(Paddle::new(Side::Left))
        .with(right_transform)
        .build();
}


impl SimpleState for Pong{
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // Get data of world
        let world = data.world;
        // Set up the camer for the world
        initialise_paddles(world);
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

/// Was last at middle of page to implement on line:
///
/// Let's run our blank screen game!
