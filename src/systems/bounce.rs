use amethyst::{
    core::{Transform},
    derive::SystemDesc,
    ecs::{Join, ReadStorage, System, SystemData, WriteStorage}, shred::Write,
};

use crate::pong::{Ball, Paddle, ARENA_HEIGHT, Side, MatBuffer};

#[derive(SystemDesc)]
pub struct BounceSystem {
    pub prev_collision: bool,
}

impl BounceSystem {
    pub fn new() -> Self {
        Self { prev_collision: false }
    }
}

impl<'s> System<'s> for BounceSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Paddle>,
        ReadStorage<'s, Transform>,
        Write<'s, MatBuffer>
    );

    fn run(&mut self, (mut balls, paddles, transforms, mut mat_buffer): Self::SystemData) {
        let mut cur_collision = false;
        for (ball, transform) in (&mut balls, &transforms).join() {
            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;

            if ball_y <= ball.radius && ball.velocity.y < 0.0 {
                ball.velocity.y *= -1.0;
            }
            if ball_y >= ARENA_HEIGHT - ball.radius && ball.velocity.y > 0.0 {
                ball.velocity.y *= -1.0;
            }

            for (paddle, paddle_transform) in (&paddles, &transforms).join() {
                let paddle_x = paddle_transform.translation().x - paddle.width / 2.0;
                let paddle_y = paddle_transform.translation().y - paddle.height / 2.0;

                if point_in_rect(
                    ball_x,
                    ball_y,
                    paddle_x - ball.radius,
                    paddle_x + paddle.width + ball.radius,
                    paddle_y + paddle.height + ball.radius,
                    paddle_y - ball.radius,
                ) {
                    cur_collision = true;
                    if paddle.side == Side::Left {
                        ball.velocity.x = ball.velocity.x.abs();
                        if !self.prev_collision {
                            mat_buffer.i_left += 1;
                        }
                    }
                    else {
                        ball.velocity.x = -ball.velocity.x.abs();
                        if !self.prev_collision {
                            mat_buffer.i_right += 1;
                        }
                    }
                }
            }
        }
        self.prev_collision = cur_collision;
    }
}

fn point_in_rect(px: f32, py: f32, left: f32, right: f32, top: f32, bottom: f32) -> bool {
    px >= left && px <= right && py >= bottom && py <= top
}