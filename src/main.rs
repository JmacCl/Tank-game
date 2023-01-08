mod arena;
mod story;
mod pause;
mod tank_customise;
mod menu;
mod systems;

use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};
use amethyst::ui::{RenderUi, UiBundle};
use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};
use amethyst::ecs::world::Generation;
use amethyst::ui::{Anchor, FontAsset, get_default_font, Interactable, LineMode, TtfFormat, UiText, UiTransform};

use core::default::Default;
use amethyst::core::TransformBundle;
use amethyst::input::{InputBundle, StringBindings};
use crate::menu::StartMenu;
// use crate::systems::SimpleButtonSystem;

fn main() -> amethyst::Result<()>{

    //Default::default()
    // Add logger so that we can see errors while code is running
    amethyst::start_logger(Default::default());

    // Setting up display configuration
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    // set up root for assets
    // let binding_path = app_root.join("config").join("bindings.ron");

    // let input_bundle = InputBundle::<StringBindings>::new()
    //     .with_bindings_from_file(binding_path)?;


    // Setting up basic application setup

    // Instance of central repo for all game logic
    let game_data = GameDataBuilder::default()
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
            // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default())
        )?
        // .with_bundle(UiBundle::<StringBindings>::new())?
        ;
        // .with_system_desc(systems::SimpleButtonSystemDesc::default(),
        //                   "simple_button_description",
        //                   &[])
        // .with(systems::SimpleButtonSystem, "button_system", &[simple_button_description])?;

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, StartMenu::default(), game_data)?;
    game.run();


    Ok(())
}
