use gamerplex_math::Vector3;

use crate::{EntityId, World};

pub struct RaycastResult {
    pub entity: EntityId,
    pub distance: f32,       // Distance from ray origin
    pub point: Vector3,      // World space hit point 
    pub normal: Vector3,     // Surface normal at hit point
}

impl World {
    pub fn raycast(
        &self, 
        origin: Vector3, 
        direction: Vector3, 
        max_distance: f32,
        filter_groups: Option<u32>
    ) -> Option<RaycastResult> {
        // Will be implemented when integrated with Rapier
        None
    }
    
    pub fn overlap_sphere(
        &self, 
        center: Vector3, 
        radius: f32,
        filter_groups: Option<u32>
    ) -> Vec<EntityId> {
        // Will be implemented when integrated with Rapier
        Vec::new()
    }
}