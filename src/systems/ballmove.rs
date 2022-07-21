use crate::pong::{Pause, PauseState};
use amethyst::core::{Transform, Time};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, Write, ReadStorage, System, SystemData, WriteStorage};

use crate::pong::{Ball, BallPos};

#[derive(SystemDesc)]
pub struct BallMoveSystem;

impl<'s> System<'s> for BallMoveSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Ball>,
        Read<'s, Time>,
        Read<'s, Pause>,
        Write<'s, BallPos>
    );

    fn run(&mut self, (mut transforms, balls, time, pause, mut ballpos): Self::SystemData) {
        if pause.state == PauseState::Paused {
            return
        }
        
        for (ball, transform) in (&balls, &mut transforms).join() {
            transform.prepend_translation_x(ball.velocity.x * time.delta_seconds());
            transform.prepend_translation_y(ball.velocity.y * time.delta_seconds());
            transform.append_rotation_z_axis(ball.velocity.x * time.delta_seconds() * 0.1);
            ballpos.x = transform.translation().x;
            ballpos.y = transform.translation().y;
        }
    }
}
