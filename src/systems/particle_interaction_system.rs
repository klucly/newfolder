use crate::pong::{Pause, PauseState};
use amethyst::core::math::Vector2;
use amethyst::core::{Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};

use crate::pong::{Particle, Ball};

#[derive(SystemDesc)]
pub struct ParticleInteractionSystem;

impl<'s> System<'s> for ParticleInteractionSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Particle>,
        ReadStorage<'s, Ball>,
        Read<'s, Pause>
    );

    fn run(&mut self, (transforms, mut particles, balls, pause): Self::SystemData) {
        if pause.state == PauseState::Paused {
            return
        }

        let ball_strength = 10f32;

        for (particle, particle_transform) in (&mut particles, &transforms).join() {

            let mut influation = Vector2::<f32>::zeros();
            for (ball, ball_transform) in (&balls, &transforms).join() {

                if  particle_transform.translation().x == ball_transform.translation().x ||
                    particle_transform.translation().y == ball_transform.translation().y
                { continue }

                let distx = particle_transform.translation().x - ball_transform.translation().x;
                let disty = particle_transform.translation().y - ball_transform.translation().y;

                let dist = distx * distx + disty * disty;

                influation.x += ball_strength/ (dist+ball_strength) * ball.velocity.x;
                influation.y += ball_strength/ (dist+ball_strength) * ball.velocity.y;
            }

            particle.starting_velocity = particle.starting_velocity * 0.98  + influation * 0.02;
        }
    }
}
