use amethyst::derive::SystemDesc;
use amethyst::ecs::{Write, Read, System, SystemData};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::shrev::EventChannel;

use crate::pong::{GameState, GameStateEvent, PauseState, Pause, MenuState, Menu, ContinueEventSystem};

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
        Write<'s, ContinueEventSystem>
    );

    fn run(&mut self, (input, mut game_state, mut pause, mut menu, mut con): Self::SystemData) {
        let current_pause_state = input.action_is_down("pause").expect("Can't find pause binding");
        let current_menu_state = input.action_is_down("menu").expect("Can't find menu binding");
        let event_channel = &mut game_state.event_channel;

        if current_pause_state && !self.current_pause_state && !self.on_menu {
            if self.is_paused {
                self.unpause(event_channel);
            } else {
                self.pause(event_channel);
            }
        }
        if current_menu_state && !self.current_menu_state {
            if self.on_menu {
                self.menu_off(event_channel);
            } else {
                self.menu_on(event_channel);
                self.pause(event_channel);
            }
        }

        pause.state = match self.is_paused {
            true => PauseState::Paused,
            false => PauseState::Unpaused,
        };

        menu.state = match self.on_menu {
            true => MenuState::MenuOn,
            false => MenuState::MenuOff,
        };

        for _event in con.read() {
            pause.state = PauseState::Unpaused;
            menu.state = MenuState::MenuOff;
            self.unpause(event_channel);
            self.menu_off(event_channel);
        }

        self.current_pause_state = current_pause_state;
        self.current_menu_state = current_menu_state;
    }
}

impl StateChangeSystem {
    fn pause(&mut self, event_channel: &mut EventChannel<GameStateEvent>) {
        if !self.is_paused {
            event_channel.single_write(GameStateEvent::Pause);
        }
        self.is_paused = true;
    }
    fn unpause(&mut self, event_channel: &mut EventChannel<GameStateEvent>) {
        if self.is_paused {
            event_channel.single_write(GameStateEvent::Unpause);
        }
        self.is_paused = false;
    }
    fn menu_on(&mut self, event_channel: &mut EventChannel<GameStateEvent>) {
        event_channel.single_write(GameStateEvent::MenuOn);
        self.on_menu = true;
    }
    fn menu_off(&mut self, event_channel: &mut EventChannel<GameStateEvent>) {
        event_channel.single_write(GameStateEvent::MenuOff);
        self.on_menu = false;
    }
}