mod mainmenu;
mod gamestate;
mod exampletile;
mod pause;
mod systems;
mod game_data;
use crate::mainmenu::MainMenuState;
use crate::exampletile::ExampleTile;
use crate::game_data::CustomGameDataBuilder;
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

    let binding_path = app_root.join("config").join("input.ron");
    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;

    let mut app_builder = Application::build(assets_dir, MainMenuState::default())?;
    let game_data = CustomGameDataBuilder::default()
        .with_base_bundle(&mut app_builder.world, TransformBundle::new())?
        .with_base_bundle(&mut app_builder.world, input_bundle)?
        .with_base_bundle(&mut app_builder.world, UiBundle::<StringBindings>::new())?
        .with_base_bundle(&mut app_builder.world, HotReloadBundle::default())?
        .with_base_bundle(&mut app_builder.world, FpsCounterBundle)?
        //.with_running(systems::MapMovementSystem::default(), "MapMovementSystem", &["input_system"])
        //.with_running(systems::CameraSwitchSystem::default(), "camera_switch", &["input_system"])
        //.with_running(systems::CameraMovementSystem::default(), "movement", &["camera_switch"])
        //.with_system_desc(crate::systems::MainMenuUiEventHandlerSystemDesc::default(),"ui_event_handler", &[])
        .with_base_bundle(
            &mut app_builder.world, 
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderUi::default())
                .with_plugin(RenderDebugLines::default())
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderTiles2D::<ExampleTile, MortonEncoder>::default())
                
        )?;

    
    let mut game = app_builder.build(game_data)?;

    game.run();

    Ok(())
}