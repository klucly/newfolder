use amethyst::derive::SystemDesc;
use amethyst::ecs::{Write, Read, System, SystemData};
use amethyst::input::{InputHandler, StringBindings};

use crate::pong::{GameState, GameStateEvent, PauseState, Pause, MenuState, Menu};

#[derive(SystemDesc)]
pub struct StateChangeSystem {
    pub current_pause_state: bool,
    pub current_menu_state: bool,
    pub is_paused: bool,
    pub on_menu: bool,
}

impl StateChangeSystem {
    pub fn new() -> Self {
        Self {
            current_pause_state: false,
            current_menu_state: false,
            is_paused: true,
            on_menu: true,
        }
    }
}

impl<'s> System<'s> for StateChangeSystem {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        Write<'s, GameState>,
        Write<'s, Pause>,
        Write<'s, Menu>,
    );

    fn run(&mut self, (input, mut game_state, mut pause, mut menu): Self::SystemData) {
        let current_pause_state = input.action_is_down("pause").expect("Can't find pause binding");
        let current_menu_state = input.action_is_down("menu").expect("Can't find menu binding");

        if current_pause_state && !self.current_pause_state && !self.on_menu {
            self.is_paused = !self.is_paused;
            game_state.event_channel.single_write(
                match self.is_paused {
                    true => GameStateEvent::Pause,
                    false => GameStateEvent::Unpause,
                }
            );

            pause.state = match self.is_paused {
                true => PauseState::Paused,
                false => PauseState::Unpaused,
            }
        }

        if current_menu_state && !self.current_menu_state {
            self.on_menu = !self.on_menu;
            game_state.event_channel.single_write(
                match self.on_menu {
                    true => GameStateEvent::MenuOn,
                    false => GameStateEvent::MenuOff,
                }
            );

            menu.state = match self.on_menu {
                true => MenuState::MenuOn,
                false => MenuState::MenuOff,
            }
        }

        self.current_pause_state = current_pause_state;
        self.current_menu_state = current_menu_state;
    }
}
