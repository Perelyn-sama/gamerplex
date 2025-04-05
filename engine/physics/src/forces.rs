use gamerplex_math::Vector3;

use crate::{EntityId, World};

impl World {
    pub fn apply_force(&mut self, entity: EntityId, force: Vector3) -> bool {
        if let Some(handle) = self.entity_body_map.get(&entity) {
            // Will apply force when integrated with Rapier
            true
        } else {
            false
        }
    }
    
    pub fn apply_impulse(&mut self, entity: EntityId, impulse: Vector3) -> bool {
        if let Some(handle) = self.entity_body_map.get(&entity) {
            // Will apply impulse when integrated with Rapier  
            true
        } else {
            false
        }
    }
    
    pub fn set_linear_velocity(&mut self, entity: EntityId, velocity: Vector3) -> bool {
        if let Some(handle) = self.entity_body_map.get(&entity) {
            // Will set velocity when integrated with Rapier
            true
        } else {
            false
        }
    }
}