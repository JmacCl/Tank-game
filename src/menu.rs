use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};
use amethyst::ecs::world::Generation;
use amethyst::ui::{Anchor, FontAsset, get_default_font, Interactable, LineMode, TtfFormat, UiText, UiTransform};

pub struct Menu;

// Constants for arena variables size
pub const WINDOW_HEIGHT: f32 = 100.0;
pub const WINDOW_WIDTH: f32 = 100.0;

impl SimpleState for Menu{
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // Get data of world
        let world = data.world;
        // Set up the camera for the world
        initialise_camera(world);
        // Set up button for the Game
        initialise_menu(world);

    }

}

fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    // Build into the world with entity with the camera, has width and height as defined in parameters
    world
        .create_entity()
        .with(Camera::standard_2d(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build();
}

/// Initialises the start of the menu, adds buttons
///
///
fn initialise_menu(world: &mut World){

    /* Create the transform */
    let ui_transform = UiTransform::new(
        String::from("simple_button"), // id
        Anchor::Middle,                // anchor
        Anchor::Middle,                // pivot
        0f32,                          // x
        0f32,                          // y
        0f32,                          // z
        100f32,                        // width
        30f32,                         // height
    );

    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    /* Create the text */
    let ui_text = UiText::new(
        font,                   // font
        String::from("Simple Button"), // text
        [1.0, 1.0, 1.0, 0.5],          // color
        25f32,                         // font_size
        LineMode::Single,              // line mode
        Anchor::Middle,                // alignment
    );

    /* Building the entity */
    let _ = world.create_entity()
        .with(ui_transform)
        .with(ui_text)
        .with(Interactable)
        .build();

}

