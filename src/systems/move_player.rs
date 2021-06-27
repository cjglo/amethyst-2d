use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::unimon::Player;


#[derive(SystemDesc)]
pub struct MovePlayerSystem;

impl<'s> System<'s> for MovePlayerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>
    );

    fn run(&mut self,  (mut transforms, players, input): Self::SystemData) {

        for (player, transform) in (&players, &mut transforms).join() {

            let x_movement = input.axis_value("move_x");
            let y_movement = input.axis_value("move_y");

            if let Some(mv_amount) = x_movement {
                let current_x = transform.translation().x;

                transform.set_translation_x(
                    (current_x + mv_amount)
                );
            };

            if let Some(mv_amount) = y_movement {
                let current_y = transform.translation().y;

                transform.set_translation_y(
                    (current_y + mv_amount)
                );
            };
            
        }
    }
}