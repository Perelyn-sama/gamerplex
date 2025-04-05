use gamerplex_math::Vector3;

use crate::{handles::EntityId, world::World};

#[derive(Clone, Debug)]
pub enum CollisionEventType {
    Started,  // Objects just started touching
    Ongoing,  // Objects continue to touch
    Ended,    // Objects separated
}

#[derive(Clone, Debug)]
pub struct CollisionEvent {
    pub entity_a: EntityId,
    pub entity_b: EntityId,
    pub event_type: CollisionEventType,
    pub contact_point: Vector3,  // World space contact point
    pub normal: Vector3,         // Contact normal (direction)
    pub impulse: f32,            // Collision impulse magnitude
}

impl World {
    // add this to fns to World defined in events
    pub fn collision_events(&self) -> &[CollisionEvent] {
        &self.collision_events
    }
    
    pub fn clear_events(&mut self) {
        self.collision_events.clear();
    }
}