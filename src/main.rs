mod arena;
mod story;
mod pause;
mod tank_customise;
mod menu;

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

use core::default::Default;
use amethyst::core::TransformBundle;
use amethyst::input::StringBindings;
use crate::menu::Menu;

fn main() -> amethyst::Result<()>{

    //Default::default()
    // Add logger so that we can see errors while code is running
    amethyst::start_logger(Default::default());

    // Setting up display configuration
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    // set up root for assets
    let assets_dir = app_root.join("assets/");


    // Setting up basic application setup

    // Instance of central repo for all game logic
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
            // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(UiBundle::<StringBindings>::new())?
        ;
    let mut game = Application::new(assets_dir, Menu, game_data)?;
    game.run();


    Ok(())
}
