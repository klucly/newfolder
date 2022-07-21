#![windows_subsystem = "windows"]

mod pong;
use pong::{Pong};
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
use amethyst::core::transform::TransformBundle;
use amethyst::input::{InputBundle, StringBindings};

fn main() -> amethyst::Result<()> {
    // Starting logger to get any error/warning that may occur in console
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let binding_path = app_root.join("config").join("bindings.ron");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()

            // Window control
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)?
                    .with_clear([0., 0., 0., 1.])
            )

            // Rendering
            .with_plugin(RenderFlat2D::default())
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        .with(systems::BallMoveSystem, "ball_control_system", &[])
        .with(systems::BounceSystem::new(), "bounce_system", &[])
        .with(systems::BallSpeedUp, "ball_speed_up", &[])
        .with(systems::ParticleSystem, "particle_system", &[])
        .with(systems::ParticleInteractionSystem, "particle_interaction_system", &[])
        .with(systems::WinCheckSystem, "win_check_system", &[])
        .with(systems::WinningSystem::new(), "winning_system", &["win_check_system"])
        .with(systems::ScoreMovementSystem, "score_movement_system", &[])
        .with(systems::BallSpeedUiSystem, "ball_speed_ui_system", &[])
        .with(systems::StateChangeSystem::new(), "state_change_system", &[])
        .with(systems::PauseSystem, "pause_system", &[])
        .with(systems::TextRenderSupportSystem, "text_render_support_system", &[])
        .with(systems::TextRenderSupportSupportSystem, "text_render_support_support_system", &[])
        .with(systems::MenuSystem, "menu_system", &[])
        .with(systems::MouseHandleSystem, "mouse_handle_system", &[])
        .with(systems::PlayerChangeSystem, "player_change_system", &[]);

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, Pong, game_data)?;

    game.run();

    Ok(())
}