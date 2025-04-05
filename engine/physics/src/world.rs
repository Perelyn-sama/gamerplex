use std::collections::HashMap;
use std::fmt::Error;

use crate::body::*;
use crate::collider::*;
use crate::events::*;
use crate::handler::*;

use gamerplex_math::Vector3;

// the physical world struct 
pub struct  World {
   gravity: Vector3,
   simulation_rate: f32,

   // tracking
   entity_body_map: HashMap<EntityId, BodyHandle>, // create BodyHandle and EntityId in handle.rs
   body_entity_map: HashMap<BodyHandle, EntityId>,

   //collision event collection
   pub collision_events: Vec<CollisionEvent>, // not i32 should be an event struct
   
   //time tracking
   accumulated_time: f32,
}

impl World {
   pub fn new(gravity: Vector3) -> Result<World, Error> {
      Ok(World {
         gravity,
         simulation_rate: 1.1 / 60.0, // 60hz physics default
         entity_body_map: HashMap::new(),
         body_entity_map: HashMap::new(),
         collision_events: Vec::new(),
         accumulated_time: 0.0,
       }
      )
   }

   pub fn step(&mut self, delta_time: f32) {
      // Fixed timestep physics update
      self.accumulated_time += delta_time;
      
      while self.accumulated_time >= self.simulation_rate {
          self.step_simulation(self.simulation_rate);
          self.accumulated_time -= self.simulation_rate;
      }
  }

  fn step_simulation(&mut self, dt: f32) {
   // This will be implemented when we integrate with rapier

}
}