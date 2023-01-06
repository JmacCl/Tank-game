use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};
use amethyst::core::Hidden;
use amethyst::ecs::Entity;
use amethyst::ecs::world::Generation;
use amethyst::ui::{Anchor, FontAsset, get_default_font, Interactable, LineMode, TtfFormat, UiEventType, UiImage, UiText, UiTransform};


#[derive(Default)]
pub struct Menu {
    button: Option<Entity>,
}

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
        world.register::<Interactable>();
        world.register::<UiImage>();


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
        let btn = world.create_entity()
            .with(ui_transform)
            .with(ui_text)
            .with(Interactable)
            .build();

        self.button = Some(btn);



    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent) -> SimpleTrans {
        if let StateEvent::Ui(ui_event) = event {
            let is_target = ui_event.target == self.button.unwrap();

            match ui_event.event_type {
                UiEventType::Click if is_target => {
                    /* . . . */
                },
                _ => {
                    return SimpleTrans::None;
                },
            };
        }

        SimpleTrans::None
    }

    fn on_pause(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let mut hiddens = world.write_storage::<Hidden>();

        if let Some(btn) = self.button {
            let _ = hiddens.insert(btn, Hidden);
        }
    }

    fn on_resume(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let mut hiddens = world.write_storage::<Hidden>();

        if let Some(btn) = self.button {
            let _ = hiddens.remove(btn);
        }
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
fn initialise_menu(world: &mut World) -> Entity {

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
    let btn = world.create_entity()
        .with(ui_transform)
        .with(ui_text)
        .with(Interactable)
        .build();

    return btn;

}

