use crate::pong::{Pause, PauseState};
use crate::pong::{UiScore, Side, Score, FixedWidget};
use amethyst::core::{Time, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};

#[derive(SystemDesc)]
pub struct ScoreMovementSystem;

impl<'s> System<'s> for ScoreMovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, UiScore>,
        WriteStorage<'s, FixedWidget>,
        Read<'s, Time>,
        Read<'s, Score>,
        Read<'s, Pause>
    );

    fn run(&mut self, (mut transforms, ui_scores, mut text_widgets, time, score, pause): Self::SystemData) {
        if pause.state == PauseState::Paused && time.absolute_real_time_seconds() != 0.0 {
            return
        }

        let current_time = time.absolute_real_time_seconds() as f32;
        let mut i = 0.;

        for (transform, ui_score, text_widget) in (&mut transforms, &ui_scores, &mut text_widgets).join() {
            i += 1.;

            transform.translation_mut().x += (current_time+i*42.384).sin() * time.delta_real_seconds();
            transform.translation_mut().y += (current_time/1.1+i*42.384).cos() * time.delta_real_seconds();

            transform.append_rotation_z_axis((current_time+i*42.384).cos()/1600.0);

            text_widget.text = match ui_score.side {
                Side::Left => score.player1.to_string(),
                Side::Right => score.player2.to_string(),
            }
        }
    }
}