mod mainmenu;
use crate::mainmenu::MainMenuState;
mod menuevents;

use amethyst::{
    core::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
        RenderFlat2D,
    },
    utils::application_root_dir,
    ui::{RenderUi, UiBundle},
    input::{InputBundle, StringBindings},
 };

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let assets_dir = app_root.join("assets/");

    let binding_path = app_root.join("config").join("bindings.ron");
    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_system_desc(crate::menuevents::UiEventHandlerSystemDesc::default(),"ui_event_handler", &[])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?;

    let mut game = Application::new(assets_dir, MainMenuState::default(), game_data)?;
    game.run();

    Ok(())
}