use crate::pong::Continue;
use crate::pong::ContinueEventSystem;
use crate::pong::Exit;
use crate::pong::ExitEventSystem;
use crate::pong::Score;
use crate::pong::Ball;
use crate::pong::Menu;
use crate::pong::MenuState;
use crate::pong::{Button, ButtonType, ButtonEventSystem, ARENA_HEIGHT};
use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Write, Join, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::shred::Read;
use amethyst::winit::MouseButton;

use crate::systems::WinningSystem;

#[derive(SystemDesc)]
pub struct MouseHandleSystem;

impl<'s> System<'s> for MouseHandleSystem {
    type SystemData = (
        ReadStorage<'s, Button>,
        Read<'s, InputHandler<StringBindings>>,
        Write<'s, ButtonEventSystem>,
        Read<'s, Menu>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Ball>,
        Write<'s, Score>,
        Write<'s, ContinueEventSystem>,
        Write<'s, ExitEventSystem>
    );

    fn run(&mut self, (buttons, input, mut button_event_system, menu, transforms, balls, score, mut con, mut exit): Self::SystemData) {
        let bot_buttons = [
            ButtonType::P1You,
            ButtonType::P1Bot,
            ButtonType::P1Mat,
            ButtonType::P2You,
            ButtonType::P2Bot,
            ButtonType::P2Mat,
        ];

        if input.mouse_button_is_down(MouseButton::Left) && menu.state == MenuState::MenuOn {
            if let Some((mut x, mut y)) = input.mouse_position() {
                x = x / 5.0;
                y = -y / 5.0 + ARENA_HEIGHT;

                for (button,) in (&buttons,).join() {
                    if point_in_rect(x, y, button.pos.x, button.pos.x + button.size.x, button.pos.y + button.size.y, button.pos.y) {
                        if bot_buttons.contains(&button.but_type) {
                            button_event_system.event_channel.single_write(button.but_type.clone());
                        } else {
                            match button.but_type {
                                ButtonType::Reset => WinningSystem::full_reset(balls, transforms, score),
                                ButtonType::Ok => con.event_channel.single_write(Continue::Continue),
                                ButtonType::Exit => exit.event_channel.single_write(Exit::Exit),
                                _ => ()
                            }
                            return;
                        }
                    }
                }
            }
        }


    }
}
fn point_in_rect(px: f32, py: f32, left: f32, right: f32, top: f32, bottom: f32) -> bool {
    px >= left && px <= right && py >= bottom && py <= top
}