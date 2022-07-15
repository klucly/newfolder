use crate::pong::{Pause, PauseState};
use amethyst::core::{Time};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, System, SystemData, WriteStorage};

use crate::pong::{Ball};

#[derive(SystemDesc)]
pub struct BallSpeedUp;

impl<'s> System<'s> for BallSpeedUp {
    type SystemData = (
        WriteStorage<'s, Ball>,
        Read<'s, Time>,
        Read<'s, Pause>
    );

    fn run(&mut self, (mut balls, time, pause): Self::SystemData) {
        if pause.state == PauseState::Paused {
            return
        }

        for (ball,) in (&mut balls,).join() {
            ball.velocity.x *= 1.0+time.delta_seconds()*0.01;
            ball.velocity.y *= 1.0+time.delta_seconds()*0.01;
        }
    }
}
