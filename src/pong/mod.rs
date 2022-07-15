mod default_lib;
pub use default_lib::*;

use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::{transform::Transform, math::Vector2},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    input::is_close_requested,
};
use rand::Rng;

pub struct Pong;

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handle = load_sprite_sheet(world);
        let font_spritesheet = load_spritesheet("texture\\font.png", "texture\\font.ron", world);
            
        world.register::<Paddle>();
        world.register::<Ball>();
        world.register::<Particle>();
        world.register::<UiScore>();
        world.register::<SpeedUi>();
        world.register::<FixedLetter>();
        world.register::<FixedWidget>();
        world.register::<PauseElement>();
        world.register::<MenuElement>();
        world.register::<Button>();

        initialise_event_system(world, 3);
        world.insert(Score::default());
        world.insert(Pause::default());
        world.insert(Menu::default());
        world.insert(PlayerType::default());

        initialise_paddles(world, sprite_sheet_handle.clone());
        initialise_menu(world, sprite_sheet_handle.clone(), font_spritesheet.clone());
        initialise_ball(world, sprite_sheet_handle.clone());
        initialise_particle_system(world, sprite_sheet_handle.clone());
        initialise_score(world, font_spritesheet.clone());
        initialise_speed_ui(world, font_spritesheet.clone());
        initialise_pause_elements(world, font_spritesheet.clone());
        initialise_camera(world);
    }

    fn handle_event(
            &mut self,
            _data: StateData<'_, GameData<'_, '_>>,
            event: StateEvent,
        ) -> SimpleTrans {
        
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) {
                return Trans::Quit;
            }
        }
        return Trans::None
    }

}


fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}


fn initialise_paddles(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();
    
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);

    // Repositionize paddles
    let y = ARENA_HEIGHT / 2.0;
    left_transform.set_translation_xyz(PADDLE_WIDTH, y, 0.0);
    right_transform.set_translation_xyz(ARENA_WIDTH - PADDLE_WIDTH, y, 0.0);

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Paddle::new(Side::Left))
        .with(left_transform)
        .build();

    world
        .create_entity()
        .with(sprite_render)
        .with(Paddle::new(Side::Right))
        .with(right_transform)
        .build();
}


fn initialise_ball(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {


    let mut transform = Transform::default();

    let sprite_render = SpriteRender::new(sprite_sheet_handle, 1);
    let x = ARENA_WIDTH / 2.0;
    let y = ARENA_HEIGHT / 2.0;

    transform.set_translation_x(x);
    transform.set_translation_y(y);

    world
        .create_entity()
        .with(sprite_render)
        .with(Ball::new(30.0, 30.0))
        .with(transform)
        .build();
}

fn initialise_particle_system(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let rng = &mut rand::thread_rng();
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 2);

    let max_lifetime = 60f32;
    let min_lifetime = 10f32;
    let max_speed = 5f32;

    for _ in 0..PARTICLE_COUNT {
        let vel_x = rng.gen_range(-max_speed..max_speed);
        let vel_y = rng.gen_range(-max_speed..max_speed);

        let lifetime = rng.gen_range(min_lifetime..max_lifetime);
        let mut pos = Transform::default();
        pos.set_translation_x(100000.0);
        
        world
            .create_entity()
            .with(sprite_render.clone())
            .with(Particle::new(vel_x, vel_y, 0.0, lifetime))
            .with(pos)
            .build();
    }
}

fn initialise_menu(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>, font_spritesheet: Handle<SpriteSheet>) {
    let options = TextUiOptions::default();

    let label_w1 = TextUi::generate(String::from("1"), world, font_spritesheet.clone(), options.clone());

    let mut transform1 = Transform::default();
    transform1.translation_mut().x = 38.5;
    transform1.translation_mut().y = 67.;

    world
        .create_entity()
        .with(label_w1)
        .with(MenuElement)
        .with(transform1)
        .build();

    let label_w2 = TextUi::generate(String::from("2"), world, font_spritesheet.clone(), options.clone());

    let mut transform2 = Transform::default();
    transform2.translation_mut().x = 54.5;
    transform2.translation_mut().y = 67.;

    world
        .create_entity()
        .with(label_w2)
        .with(MenuElement)
        .with(transform2)
        .build();

    let buttons = [
        (ButtonType::P1You, 28., 54.5     , 13., 9., 8, "You"),
        (ButtonType::P1Bot, 28., 54.5-12.0, 13., 9., 7, "Bot"),
        (ButtonType::P1Mat, 28., 54.5-24.0, 13., 9., 7, "Mat"),
        (ButtonType::P2You, 44., 54.5     , 13., 9., 8, "You"),
        (ButtonType::P2Bot, 44., 54.5-12.0, 13., 9., 7, "Bot"),
        (ButtonType::P2Mat, 44., 54.5-24.0, 13., 9., 7, "Mat"),

        (ButtonType::Ok   , 64.01, 54.51    , 9., 9., 4, ""),
        (ButtonType::Exit , 64.01, 54.51-12., 9., 9., 5, ""),
        (ButtonType::Reset, 64.01, 54.51-24., 9., 9., 6, ""),
    ];


    for (button_type, x, y, w, h, sprite_n, text) in buttons {

        if text.len() > 0 {
            let label = TextUi::generate(String::from(text), world, font_spritesheet.clone(), options.clone());
        
            let mut text_transform = Transform::default();
            text_transform.translation_mut().x = x + w / 2.0 + 4.01;
            text_transform.translation_mut().y = y + h / 2.0 + 0.01;
        
            world
                .create_entity()
                .with(label)
                .with(MenuElement)
                .with(text_transform)
                .build();
        }

        let render = SpriteRender::new(sprite_sheet_handle.clone(), sprite_n);
        let mut transform = Transform::default();
        transform.translation_mut().x = x + w / 2.0;
        transform.translation_mut().y = y + h / 2.0;

        let button = Button {
            pos: Vector2::new(x, y),
            size: Vector2::new(w, h),
            but_type: button_type
        };

        world
            .create_entity()
            .with(button)
            .with(transform)
            .with(MenuElement)
            .with(render.clone())
            .build();
    }
            
    let menu_sprite_render = SpriteRender::new(sprite_sheet_handle.clone(), 3);
    let mut menu_transform = Transform::default();
    menu_transform.translation_mut().x = ARENA_WIDTH / 2.0;
    menu_transform.translation_mut().y = ARENA_HEIGHT / 2.0;

    world
        .create_entity()
        .with(menu_sprite_render.clone())
        .with(MenuElement)
        .with(menu_transform)
        .build();

}

fn initialise_event_system(world: &mut World, event_readers_count: u32) {
    let mut game_state = GameState {
        event_channel: EventChannel::new(),
        readers: vec![]
    };

    for _ in 0..event_readers_count {
        game_state.register();
    }

    world.insert(game_state);

    let mut event_channel = EventChannel::new();
    let button = ButtonEventSystem {
        reader: vec![event_channel.register_reader()],
        event_channel: event_channel,
    };

    world.insert(button);

    let mut event_channel = EventChannel::new();
    let con = ContinueEventSystem {
        reader: vec![event_channel.register_reader()],
        event_channel: event_channel,
    };

    world.insert(con);


}

fn initialise_score(world: &mut World, font_spritesheet: Handle<SpriteSheet>) {

    let mut options = TextUiOptions::default();
    options.letter_size.x = 3.0;
    options.letter_size.y = 3.0;
    options.letter_offset.x = 3.0 * 4.0;

    let label_w1 = TextUi::generate(String::from("0  "), world, font_spritesheet.clone(), options.clone());

    let mut transform1 = Transform::default();
    transform1.translation_mut().x = 25.;
    transform1.translation_mut().y = ARENA_HEIGHT - 15.;

    world
        .create_entity()
        .with(label_w1)
        .with(UiScore::left())
        .with(transform1)
        .build();


    let label_w2 = TextUi::generate(String::from("0  "), world, font_spritesheet, options.clone());

    let mut transform2 = Transform::default();
    transform2.translation_mut().x = ARENA_WIDTH;
    transform2.translation_mut().y = ARENA_HEIGHT - 15.;

    world
        .create_entity()
        .with(label_w2)
        .with(UiScore::right())
        .with(transform2)
        .build();
}

fn initialise_speed_ui(world: &mut World, font_spritesheet: Handle<SpriteSheet>) {
    let options = TextUiOptions::default();

    let label_w2 = TextUi::generate(String::from("  0.0"), world, font_spritesheet, options.clone());

    let mut transform2 = Transform::default();
    transform2.translation_mut().x = ARENA_WIDTH / 2.0 + 5.1;
    transform2.translation_mut().y = ARENA_HEIGHT - 15.0;

    world
        .create_entity()
        .with(label_w2)
        .with(SpeedUi)
        .with(transform2)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();

    let texture_handle = {
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/pong_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage
        )
    };

    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/pong_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store
    )
}

fn initialise_pause_elements(world: &mut World, font_spritesheet: Handle<SpriteSheet>) {

    let mut options = TextUiOptions::default();
    options.letter_size.x = 1.5;
    options.letter_size.y = 1.5;
    options.letter_offset.x = 1.5 * 4.0;

    let label_w = TextUi::generate(String::from("PAUSED"), world, font_spritesheet.clone(), options.clone());

    let mut transform = Transform::default();
    transform.translation_mut().x = ARENA_WIDTH - 20.;
    transform.translation_mut().y = 10.;

    world
        .create_entity()
        .with(label_w)
        .with(PauseElement)
        .with(transform)
        .build();
}

fn load_spritesheet(img_path: &str, ron_path: &str, world: &mut World) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();

    let texture_handle = {
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            img_path,
            ImageFormat::default(),
            (),
            &texture_storage
        )
    };

    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        ron_path,
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store
    )
}
