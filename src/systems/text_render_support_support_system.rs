use crate::pong::{FixedWidget};
use amethyst::core::{Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, ReadStorage, System, SystemData, WriteStorage};

#[derive(SystemDesc)]
pub struct TextRenderSupportSupportSystem;

impl<'s> System<'s> for TextRenderSupportSupportSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        WriteStorage<'s, FixedWidget>,
    );

    fn run(&mut self, (transforms, mut fixed_widgets): Self::SystemData) {
        for (fixed_widget, transform) in (&mut fixed_widgets, &transforms).join() {
            fixed_widget.internal_transform = transform.clone();
        }
    }
}