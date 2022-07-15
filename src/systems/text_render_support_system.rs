use crate::pong::{FixedWidget, FixedLetter};
use amethyst::core::{Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, ReadStorage, System, SystemData, WriteStorage};
use amethyst::renderer::SpriteRender;

#[derive(SystemDesc)]
pub struct TextRenderSupportSystem;

impl<'s> System<'s> for TextRenderSupportSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, FixedWidget>,
        ReadStorage<'s, FixedLetter>,
        WriteStorage<'s, SpriteRender>,
    );

    fn run(&mut self, (mut transforms, fixed_widgets, fixed_letters, mut sprite_render): Self::SystemData) {
        for (fixed_widget,) in (&fixed_widgets,).join() {
            let needed_id = fixed_widget.id;
            let internal_translation = fixed_widget.internal_transform.translation();
            let options = &fixed_widget.options;
            let angle_vec = fixed_widget.internal_transform.euler_angles().2;
            let text = fixed_widget.text.clone();
            let i_offset = text.len() as f32 / 2.0 + 0.5;

            let center_x = internal_translation.x;
            let center_y = internal_translation.y;
            let x = angle_vec.cos();
            let y = angle_vec.sin();

            for (letter_transform, letter, letter_render) in (&mut transforms, &fixed_letters, &mut sprite_render).join() {
                let current_id = letter.id;

                if current_id == needed_id {
                    letter_transform.translation_mut().x = center_x + (letter.i as f32 - i_offset) * (x * options.letter_offset.x - y * options.letter_offset.y);
                    letter_transform.translation_mut().y = center_y + (letter.i as f32 - i_offset) * (y * options.letter_offset.x + x * options.letter_offset.y);

                    letter_transform.scale_mut().x = options.letter_size.x;
                    letter_transform.scale_mut().y = options.letter_size.y;

                    letter_transform.set_rotation_2d(angle_vec);

                    let letters = text.chars().collect::<Vec<_>>();
                    if letters.len() > letter.i {
                        letter_render.sprite_number = letters[letter.i] as usize;
                    }
                    else {
                        letter_render.sprite_number = 0;
                    }
                }
            }
        }
    }
}