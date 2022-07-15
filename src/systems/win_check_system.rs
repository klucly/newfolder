use crate::pong::GameState;
use amethyst::core::{Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Write, ReadStorage, System, SystemData};

use crate::pong::{GameStateEvent, Ball, ARENA_WIDTH, BALL_RADIUS};

#[derive(SystemDesc)]
pub struct WinCheckSystem;

impl<'s> System<'s> for WinCheckSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Ball>,
        Write<'s, GameState>
    );

    fn run(&mut self, (transforms, balls, mut game_state): Self::SystemData) {
        for (_ball, transform) in (&balls, &transforms).join() {

            if transform.translation().x < BALL_RADIUS {
                game_state.event_channel.single_write(GameStateEvent::Player2Win);
            } else if transform.translation().x > ARENA_WIDTH - BALL_RADIUS {
                game_state.event_channel.single_write(GameStateEvent::Player1Win);
            }
        }
    }
}