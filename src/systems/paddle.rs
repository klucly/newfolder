use amethyst::core::{Transform, Time};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::shred::Write;

use crate::pong::{Paddle, Side, ARENA_HEIGHT, PADDLE_HEIGHT, Pause, PauseState, PlayerController, PlayerType, Ball, BallPos, ARENA_WIDTH, MatBuffer, GameStateEvent, GameState};

impl PaddleSystem {
    fn player_movement(paddle: &Paddle, input: &Read<InputHandler<StringBindings>>, transform: &mut Transform) {
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

    fn bot_movement(bally: f32, ball: &Ball, transform: &mut Transform, delta: f32) {
        let y = transform.translation().y;
        if bally + ball.velocity.y * delta > y - 2. && y < ARENA_HEIGHT - PADDLE_HEIGHT / 2.0 {
            transform.translation_mut().y += (ball.velocity.y.abs() * delta).min(1.2);
        }
        if bally + ball.velocity.y * delta < y + 2. && y > PADDLE_HEIGHT / 2.0 {
            transform.translation_mut().y -= (ball.velocity.y.abs() * delta).min(1.2);
        }
    }

    fn mat_movement(paddle: &Paddle, transform: &mut Transform, i: &mut usize) {
        let calc_y = PaddleSystem::mat_calc_pos(*i, paddle.side.clone());
        let y = transform.translation().y;
        if calc_y > y && y < ARENA_HEIGHT - PADDLE_HEIGHT / 2.0 {
            transform.translation_mut().y += 1.2;
        } if calc_y < y + 2. && y > PADDLE_HEIGHT / 2.0 {
            transform.translation_mut().y -= 1.2;
        }

        if transform.translation().y > ARENA_HEIGHT - PADDLE_HEIGHT / 2.0 {
            transform.translation_mut().y = ARENA_HEIGHT - PADDLE_HEIGHT / 2.0;
        } else if transform.translation().y < PADDLE_HEIGHT / 2.0 {
            transform.translation_mut().y = PADDLE_HEIGHT / 2.0;
        }
    }
    
    fn mat_calc_pos(i: usize, side: Side) -> f32 {
        match side {
            Side::Left => {
                return PaddleSystem::mat_calc_y(132. + i as f32 * 176.0);
            },
            Side::Right => {
                return PaddleSystem::mat_calc_y(44. + i as f32 * 176.0);
            }
        }
    }

    fn mat_calc_y(mut x: f32) -> f32 {
        x -= (ARENA_WIDTH - 6.0) / 2.0;
        if x % (2. * ARENA_HEIGHT) - ARENA_HEIGHT > 0. {
            return x % ARENA_HEIGHT;
        } else {
            return -x % ARENA_HEIGHT + ARENA_HEIGHT;
        }

    }

    fn get_ball(balls: ReadStorage<Ball>) -> Option<Ball> {
        for (ball,) in (&balls,).join() {
            return Some((*ball).clone());
        }
        None
    }
}

#[derive(SystemDesc)]
pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Pause>,
        Read<'s, PlayerController>,
        Read<'s, BallPos>,
        ReadStorage<'s, Ball>,
        Read<'s, Time>,
        Write<'s, MatBuffer>,
        Write<'s, GameState>
    );

    fn run(&mut self, (mut transforms, paddles, input, pause, player_controller, ballpos, balls, time, mut mat_buffer, mut game_state): Self::SystemData) {
        if pause.state == PauseState::Paused {
            return
        }

        for event in game_state.read(3) {
            if event == &GameStateEvent::Player1Win || event == &GameStateEvent::Player2Win {
                mat_buffer.i_left = 0;
                mat_buffer.i_right = 0;
            }
        }
        
        let ball = PaddleSystem::get_ball(balls).unwrap();
        let delta = time.delta_real_seconds();
        
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            match paddle.side {
                Side::Left => {
                    match player_controller.player1 {
                        PlayerType::You => PaddleSystem::player_movement(paddle, &input, transform),
                        PlayerType::Bot => PaddleSystem::bot_movement(ballpos.y, &ball, transform, delta),
                        PlayerType::Mat => PaddleSystem::mat_movement(paddle, transform, &mut mat_buffer.i_left)
                    }
                }
                Side::Right => {
                    match player_controller.player2 {
                        PlayerType::You => PaddleSystem::player_movement(paddle, &input, transform),
                        PlayerType::Bot => PaddleSystem::bot_movement(ballpos.y, &ball, transform, delta),
                        PlayerType::Mat => PaddleSystem::mat_movement(paddle, transform, &mut mat_buffer.i_right)
                    }
                }
            }
        }
    }
}
