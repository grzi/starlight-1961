use amethyst::core::ecs::{System, ReadStorage, WriteStorage, Entities, Read, Join};
use crate::entities::doors::{PlasmaDoor, DoorState};
use amethyst::renderer::SpriteRender;
use std::collections::HashMap;
use amethyst::core::Time;
use rand::Rng;
use crate::utils::sprites::plasma_doors::{plasma_door_next_sprite, plasma_door_close_sprite};

const timing_change_sprite:f32 = 0.2;

pub struct PlasmaDoorSystem {
    sprite_changing_timer: f32,
    door_timers: HashMap<u32, f32>
}


impl Default for PlasmaDoorSystem {
    fn default() -> Self {
        PlasmaDoorSystem {
            sprite_changing_timer: timing_change_sprite,
            door_timers: HashMap::new()
        }
    }
}


impl<'s> System<'s> for PlasmaDoorSystem {
    type SystemData = (
        WriteStorage<'s, PlasmaDoor>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, Time>,
        Entities<'s>);

    fn run(&mut self, (mut doors, mut sprites, time, entities): Self::SystemData) {
        for (door, entity, sprite) in (&mut doors, &entities, &mut sprites).join(){
            *self.door_timers.entry(entity.id())
                .or_insert(3.5) -= time.delta_seconds();
            match door.state {
                DoorState::Closed => {
                    self.sprite_changing_timer -= time.delta_seconds();
                    if self.sprite_changing_timer <= 0. {
                        sprite.sprite_number = plasma_door_next_sprite(sprite.sprite_number);
                        self.sprite_changing_timer = timing_change_sprite;
                    }
                },
                _ => {}
            };

            let val = self.door_timers.get(&entity.id()).unwrap();
            if val <= &0. {
                self.door_timers.get(&entity.id()).unwrap();
                match door.state {
                    DoorState::Closed => {
                        door.state = DoorState::Open;
                        sprite.sprite_number = plasma_door_close_sprite(sprite.sprite_number);
                        self.door_timers.insert(entity.id(), 1.5);
                    },
                    DoorState::Open => {
                        door.state = DoorState::Closed;
                        sprite.sprite_number = door.initial_sprite;
                        self.door_timers.insert(entity.id(), 3.5);
                    }
                };


            }
        }
    }

}