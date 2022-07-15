use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Write, Join, ReadStorage, System, SystemData, WriteStorage};

use crate::pong::{GameState, GameStateEvent, PauseElement};

#[derive(SystemDesc)]
pub struct PauseSystem;

impl<'s> System<'s> for PauseSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, PauseElement>,
        Write<'s, GameState>,
    );

    fn run(&mut self, (mut transforms, pause_elements, mut game_state): Self::SystemData) {
        for event in game_state.read(1) {
            match event {
                GameStateEvent::Unpause => {
                    for (transform, _pause_element) in (&mut transforms, &pause_elements).join() {
                        transform.translation_mut().x += 4000.0;
                    }
                }
                GameStateEvent::Pause => {
                    for (transform, _pause_element) in (&mut transforms, &pause_elements).join() {
                        transform.translation_mut().x -= 4000.0;
                    }
                }

                _ => ()
            }
        }
    }
}