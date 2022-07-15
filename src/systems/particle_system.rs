use crate::pong::{Pause, PauseState};
use amethyst::core::{Transform, Time};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, System, SystemData, WriteStorage};
use rand::Rng;

use crate::pong::{Particle, ARENA_WIDTH, ARENA_HEIGHT};

#[derive(SystemDesc)]
pub struct ParticleSystem;

impl<'s> System<'s> for ParticleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Particle>,
        Read<'s, Time>,
        Read<'s, Pause>
    );

    fn run(&mut self, (mut transforms, mut particles, time, pause): Self::SystemData) {
        if pause.state == PauseState::Paused {
            return
        }
        
        let max_speed = 5f32;

        let mut rng = rand::thread_rng();
        
        for (particle, transform) in (&mut particles, &mut transforms).join() {
            let current_time = time.absolute_real_time_seconds() as f32;

            if particle.left_time(current_time) == 0.0 {

                particle.reinit(current_time);
                transform.translation_mut().x = rng.gen_range(0f32..ARENA_WIDTH);
                transform.translation_mut().y = rng.gen_range(0f32..ARENA_HEIGHT);
                particle.starting_velocity.x = rng.gen_range(-max_speed..max_speed);
                particle.starting_velocity.y = rng.gen_range(-max_speed..max_speed);
                particle.rotational_velocity = rng.gen_range(-1f32..1f32);
            }

            let velocity = particle.get_velocity(current_time);
            transform.prepend_translation_x(velocity.x * time.delta_real_seconds());
            transform.prepend_translation_y(velocity.y * time.delta_real_seconds());
            transform.append_rotation_z_axis(particle.rotational_velocity * (particle.left_time(current_time) / particle.lifetime) * time.delta_real_seconds() * 3.0);
        }
    }
}
