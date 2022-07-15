use crate::pong::{GameStateEvent, GameState, Score};
use amethyst::core::{Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::storage::MaskedStorage;
use amethyst::ecs::{Join, Write, System, SystemData, WriteStorage, Storage};
use amethyst::shred::{FetchMut};

use crate::pong::{Ball, ARENA_WIDTH, ARENA_HEIGHT};

#[derive(SystemDesc)]
pub struct WinningSystem {
    event_reader_id: usize,
}

impl WinningSystem {
    pub fn new() -> Self {
        Self { event_reader_id: 0 }
    }
    

    fn reset(mut balls: Storage<Ball, FetchMut<MaskedStorage<Ball>>>, mut transforms: Storage<Transform, FetchMut<MaskedStorage<Transform>>>) {
        for (transform, ball) in (&mut transforms, &mut balls).join() {
            transform.translation_mut().x = ARENA_WIDTH / 2.0;
            transform.translation_mut().y = ARENA_HEIGHT / 2.0;
            ball.velocity.x = 30.0;
            ball.velocity.y = 30.0;
        }
    }
}

impl<'s> System<'s> for WinningSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Ball>,
        Write<'s, GameState>,
        Write<'s, Score>
    );

    fn run(&mut self, (transforms, balls, mut game_state, mut score): Self::SystemData) {
        for event in game_state.read(self.event_reader_id) {
            match event {
                GameStateEvent::Player1Win => {
                    score.player1 += 1;
                    WinningSystem::reset(balls, transforms);
                },
                GameStateEvent::Player2Win => {
                    score.player2 += 1;
                    WinningSystem::reset(balls, transforms);
                },
                _ => ()
            }
            return
        }
    }
}
