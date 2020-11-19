mod mainmenu;
mod gamestate;
mod exampletile;
mod pause;
use crate::mainmenu::MainMenuState;
use crate::exampletile::ExampleTile;
//mod menuevents;

use amethyst::{
    core::{TransformBundle},
    prelude::*,
    renderer::{
        plugins::RenderToWindow,
        types::DefaultBackend,
        RenderingBundle,
        RenderFlat2D,
        RenderDebugLines,
        //debug_drawing::DebugLinesComponent,
    },
    utils::{application_root_dir, fps_counter::FpsCounterBundle},
    ui::{RenderUi, UiBundle},
    input::{InputBundle, StringBindings},
    assets::HotReloadBundle,
    tiles::{MortonEncoder, RenderTiles2D},
 };



fn main() -> amethyst::Result<()> {
    amethyst::Logger::from_config(Default::default())
        .level_for("amethyst_tiles", log::LevelFilter::Warn)
        .start();
    
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
        .with_bundle(HotReloadBundle::default())?
        .with_bundle(FpsCounterBundle)?
        //.with_system_desc(crate::menuevents::UiEventHandlerSystemDesc::default(),"ui_event_handler", &[])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderUi::default())
                .with_plugin(RenderDebugLines::default())
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderTiles2D::<ExampleTile, MortonEncoder>::default())
                
        )?;

    let mut game = Application::build(assets_dir, MainMenuState::default())?.build(game_data)?;
    game.run();

    Ok(())
}