use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, ReadStorage, System, SystemData, WriteStorage};

use crate::pong::{Ball, SpeedUi, FixedWidget};

#[derive(SystemDesc)]
pub struct BallSpeedUiSystem;

impl<'s> System<'s> for BallSpeedUiSystem {
    type SystemData = (
        WriteStorage<'s, FixedWidget>,
        ReadStorage<'s, Ball>,
        ReadStorage<'s, SpeedUi>
    );

    fn run(&mut self, (mut texts, balls, speed_uis): Self::SystemData) {
        let default_speed = 30f32;

        for (ball,) in (&balls,).join() {
            for (text, _speed_ui) in (&mut texts, &speed_uis).join() {
                let ball_speed = ball.velocity.x.abs();
                let relative_speed = ball_speed / default_speed;

                text.text = format!("{relative_speed:.1}");
            }
            
            break
        }

    }
}