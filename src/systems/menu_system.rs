use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Write, Join, ReadStorage, System, SystemData, WriteStorage};

use crate::pong::{GameState, GameStateEvent, MenuElement};

#[derive(SystemDesc)]
pub struct MenuSystem;

impl<'s> System<'s> for MenuSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, MenuElement>,
        Write<'s, GameState>,
    );

    fn run(&mut self, (mut transforms, menu_elements, mut game_state): Self::SystemData) {
        for event in game_state.read(2) {
            match event {
                GameStateEvent::MenuOn => {
                    for (transform, _menu_element) in (&mut transforms, &menu_elements).join() {
                        transform.translation_mut().x += 4000.0;
                    }
                }
                GameStateEvent::MenuOff => {
                    for (transform, _menu_element) in (&mut transforms, &menu_elements).join() {
                        transform.translation_mut().x -= 4000.0;
                    }
                }

                _ => ()
            }
        }
    }
}