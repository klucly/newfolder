use crate::pong::{Button, PlayerController, ButtonType, PlayerType, ButtonEventSystem};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Write, Join, ReadStorage, System, SystemData, WriteStorage};
use amethyst::renderer::SpriteRender;

#[derive(SystemDesc)]
pub struct PlayerChangeSystem;

impl<'s> System<'s> for PlayerChangeSystem {
    type SystemData = (
        ReadStorage<'s, Button>,
        Write<'s, PlayerController>,
        WriteStorage<'s, SpriteRender>,
        Write<'s, ButtonEventSystem>,
    );

    fn run(&mut self, (buttons, mut player_controller, mut sprite_render, mut button_event_system): Self::SystemData) {
        let player_1_buttons = [
            ButtonType::P1You,
            ButtonType::P1Bot,
            ButtonType::P1Mat,
        ];
        let player_2_buttons = [
            ButtonType::P2You,
            ButtonType::P2Bot,
            ButtonType::P2Mat,
        ];
        let bot_buttons = [player_1_buttons.clone(), player_2_buttons.clone()].concat();

        let events = button_event_system.read();

        for event in events {
            for (button, sprite) in (&buttons, &mut sprite_render).join() {

                if bot_buttons.contains(&button.but_type) {
                    if &button.but_type == event {
                        sprite.sprite_number = 8;
                        if player_1_buttons.contains(&button.but_type) {
                            player_controller.player1 = match event {
                                ButtonType::P1Bot => PlayerType::Bot,
                                ButtonType::P1You => PlayerType::You,
                                ButtonType::P1Mat => PlayerType::Mat,
                                _ => PlayerType::default(),
                            }
                        } else {
                            player_controller.player2 = match event {
                                ButtonType::P2Bot => PlayerType::Bot,
                                ButtonType::P2You => PlayerType::You,
                                ButtonType::P2Mat => PlayerType::Mat,
                                _ => PlayerType::default(),
                            }
                        }
                    } else if player_1_buttons.contains(&button.but_type) && player_1_buttons.contains(&event) ||
                              player_2_buttons.contains(&button.but_type) && player_2_buttons.contains(&event) {
                        sprite.sprite_number = 7;
                    }
                }
            }
        }
    }
}