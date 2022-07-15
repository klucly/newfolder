use amethyst::core::{Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::pong::{Paddle, Side, ARENA_HEIGHT, PADDLE_HEIGHT, Pause, PauseState};

#[derive(SystemDesc)]
pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Pause>
    );

    fn run(&mut self, (mut transforms, paddles, input, pause): Self::SystemData) {
        if pause.state == PauseState::Paused {
            return
        }
        
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            let movement = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle")
            };

            if let Some(mv_amount) = movement {
                let scaled_amount = 1.2 * mv_amount as f32;

                let current_y = transform.translation().y;
                let min_dist = PADDLE_HEIGHT / 2.;
                let max_dist = ARENA_HEIGHT - PADDLE_HEIGHT / 2.;

                transform.set_translation_y((
                    scaled_amount + current_y
                    ).max(
                        min_dist
                    ).min(
                        max_dist
                    )
                );
            }
        }
    }
}
