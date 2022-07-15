use std::fmt::Debug;

use amethyst::{prelude::{World, WorldExt, Builder}, core::{Transform}, renderer::SpriteRender};
use crate::pong::Handle;
use amethyst::{
    ecs::{Component, DenseVecStorage}, shrev::{ReaderId, EventIterator}, renderer::SpriteSheet,
};
use amethyst::core::math::Vector2;
pub use amethyst::shrev::EventChannel;

pub const ARENA_HEIGHT: f32 = 100.;
pub const ARENA_WIDTH: f32 = 100.;
pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;
pub const BALL_RADIUS: f32 = 2.0;
pub const PARTICLE_COUNT: usize = 10;


#[derive(PartialEq, Eq, Debug, Default, Clone)]
pub enum Side {
    #[default]
    Left,
    Right,
}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    pub fn new(side: Side) -> Self {
        Paddle {
            side,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

pub struct Ball {
    pub velocity: Vector2<f32>,
    pub radius: f32,
}

impl Ball {
    pub fn new(vel_x: f32, vel_y: f32) -> Self {
        Ball {
            velocity: Vector2::new(vel_x, vel_y),
            radius: BALL_RADIUS,
        }
    }
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

pub struct Particle {
    pub lifetime: f32,
    pub creating_time: f32,
    pub starting_velocity: Vector2<f32>,
    pub rotational_velocity: f32,
}

impl Particle {
    pub fn new(vel_x: f32, vel_y: f32, current_time: f32, lifetime: f32) -> Self {
        Particle {
            lifetime: lifetime,
            creating_time: current_time,
            starting_velocity: Vector2::new(vel_x, vel_y),
            rotational_velocity: 0.,
        }
    }

    pub fn get_velocity(&self, current_time: f32) -> Vector2<f32> {
        let max_speed = 10f32;
        let effect: f32;

        if current_time == self.creating_time {
            effect = 1f32;
        }
        else {
            effect = self.lifetime / (current_time - self.creating_time + 2.0);
        }
        
        Vector2::new(
            (effect * self.starting_velocity.x).min(max_speed).max(-max_speed),
            (effect * self.starting_velocity.y).min(max_speed).max(-max_speed)
        )
    }

    pub fn left_time(&self, current_time: f32) -> f32 {
        (self.lifetime - (current_time - self.creating_time)).max(0.0)
    }

    pub fn reinit(&mut self, current_time: f32) {
        self.creating_time = current_time;
    }
}

#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub enum ButtonType {
    P1You, P2You,
    P1Bot, P2Bot,
    P1Mat, P2Mat,
    #[default]
    Exit,
    Ok,
    Reset,
}

#[derive(Default)]
pub struct ButtonEventSystem {
    pub event_channel: EventChannel<ButtonType>,
    pub reader: Vec<ReaderId<ButtonType>>,
}
impl ButtonEventSystem {
    pub fn read(&mut self) -> EventIterator<ButtonType> {
        self.event_channel.read(&mut self.reader[0])
    }
}

#[derive(Eq, PartialEq)]
pub enum Continue {
    Continue,
}

#[derive(Default)]
pub struct ContinueEventSystem {
    pub event_channel: EventChannel<Continue>,
    pub reader: Vec<ReaderId<Continue>>,
}
impl ContinueEventSystem {
    pub fn read(&mut self) -> EventIterator<Continue> {
        self.event_channel.read(&mut self.reader[0])
    }
}

#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub enum PlayerType {
    #[default]
    You,
    Bot,
    Mat
}

#[derive(Default, Debug)]
pub struct PlayerController {
    pub player1: PlayerType,
    pub player2: PlayerType,
}

pub struct Button {
    pub pos: Vector2<f32>,
    pub size: Vector2<f32>,
    pub but_type: ButtonType,
}
impl Component for Button {
    type Storage = DenseVecStorage<Self>;
}


impl Component for Particle {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GameStateEvent {
    Player1Win,
    Player2Win,
    Pause,
    Unpause,
    MenuOn,
    MenuOff,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub enum MenuState {
    #[default]
    MenuOn,
    MenuOff,
}

#[derive(Default, Debug, Clone)]
pub struct Menu {
    pub state: MenuState,
}

#[derive(Default)]
pub struct GameState {
    pub event_channel: EventChannel<GameStateEvent>,
    pub readers: Vec<ReaderId<GameStateEvent>>,
}

impl GameState {
    pub fn read(&mut self, id: usize) -> EventIterator<GameStateEvent> {
        self.event_channel.read(&mut self.readers[id])
    }

    pub fn register(&mut self) -> usize {
        self.readers.push(
            self.event_channel.register_reader()
        );
        
        self.readers.len()-1
    }
}

#[derive(Default, Debug, Clone)]
pub struct UiScore {
    pub side: Side,
}

impl UiScore {
    pub fn left() -> Self {
        Self { side: Side::Left }
    }
    pub fn right() -> Self {
        Self { side: Side::Right }
    }
}

impl Component for UiScore {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct Score {
    pub player1: usize,
    pub player2: usize
}

#[derive(Default)]
pub struct SpeedUi;
impl Component for SpeedUi {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub enum PauseState {
    #[default]
    Paused,
    Unpaused,
}

#[derive(Default, Debug, Clone)]
pub struct Pause {
    pub state: PauseState
}

#[derive(Clone)]
pub struct PauseElement;
impl Component for PauseElement {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone)]
pub struct MenuElement;
impl Component for MenuElement {
    type Storage = DenseVecStorage<Self>;
}

pub struct FixedLetter {
    pub id: usize,
    pub i: usize,
}
impl Component for FixedLetter {
    type Storage = DenseVecStorage<Self>;
}

pub struct FixedWidget {
    pub internal_transform: Transform,
    pub options: TextUiOptions,
    pub id: usize,
    pub text: String,
}
impl Component for FixedWidget {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone)]
pub struct TextUiOptions {
    pub letter_size: Vector2<f32>,
    pub letter_offset: Vector2<f32>,
}

impl TextUiOptions {
    pub fn new(letter_size: Vector2<f32>, letter_offset: Vector2<f32>) -> Self {
        Self { letter_size, letter_offset }
    }
}

impl Default for TextUiOptions {
    fn default() -> Self {
        TextUiOptions::new(Vector2::new(1.0, 1.0), Vector2::new(4.0, 0.0))
    }
}

pub struct TextUi;

impl TextUi {
    pub fn generate(text: String, world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>, options: TextUiOptions) -> FixedWidget {
        if !text.is_ascii() {
            panic!("Ascii only");
        }

        let oh = rand::random::<usize>();
        
        let widget = FixedWidget { id: oh, internal_transform: Transform::default(), options, text: text.clone() };
    
        for (i, letter) in text.as_bytes().iter().enumerate() {
            let transform = Transform::default();
            let render = SpriteRender::new(sprite_sheet_handle.clone(), *letter as usize);
            
            let fixed_letter = FixedLetter {
                id: oh,
                i: i,
            };
            
            world
                .create_entity()
                .with(fixed_letter)
                .with(transform)
                .with(render)
                .build();
        }
        widget
    }

    pub fn generate_size(size: usize, world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>, options: TextUiOptions) -> FixedWidget {
        let text = vec![0u8 as char; size].iter().collect::<String>();
        TextUi::generate(text, world, sprite_sheet_handle, options)
    }

}

