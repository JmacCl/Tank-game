use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};
use amethyst::winit::MouseCursor::Default;

pub struct Pong;

impl SimpleState for Pong{}

fn main() -> amethyst::Result<()>{

    // Add logger so that we can see errors while code is running
    amethyst::start_logger(Default::default());

    // Setting up display configuration
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    // Setting up basic application setup

    // INstance of central repo for all game logic
    let game_data = GameDataBuilder::default();
    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, Pong, game_data)?;
    game.run();


    Ok(())
}
