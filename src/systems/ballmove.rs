use crate::pong::{Pause, PauseState};
use amethyst::core::{Transform, Time};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};

use crate::pong::{Ball};

#[derive(SystemDesc)]
pub struct BallMoveSystem;

impl<'s> System<'s> for BallMoveSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Ball>,
        Read<'s, Time>,
        Read<'s, Pause>,
    );

    fn run(&mut self, (mut transforms, balls, time, pause): Self::SystemData) {
        if pause.state == PauseState::Paused {
            return
        }
        
        for (ball, transform) in (&balls, &mut transforms).join() {
            transform.prepend_translation_x(ball.velocity.x * time.delta_seconds());
            transform.prepend_translation_y(ball.velocity.y * time.delta_seconds());
            transform.append_rotation_z_axis(ball.velocity.x * time.delta_seconds() * 0.1);
        }
    }
}

// fn collide(a: &Transform, b: &Transform) -> bool {
//     let x1_left = a.translation().x - a.scale().x / 2.0;
//     let x2_left = b.translation().x - b.scale().x / 2.0;

//     let x1_right = x1_left + a.scale().x;
//     let x2_right = x2_left + b.scale().x;

//     if !(x1_left <= x2_right) {
//         return false
//     }

//     if !(x2_left <= x2_right) {
//         return false
//     }

//     let y1_down = a.translation().y - a.scale().y / 2.0;
//     let y2_down = b.translation().y - b.scale().y / 2.0;

//     let y1_up = y1_down + a.scale().y;
//     let y2_up = y2_down + b.scale().y;

//     if !(y1_down <= y2_up) {
//         return false
//     }

//     if !(y2_down <= y1_up) {
//         return false
//     }

//     return true
// }