use gamerplex_math::{Vector3, Quaternion};

use crate::body::{BodyType, Body};
use crate::collider::{ColliderDef, ColliderShape};
use crate::handles::{BodyHandle, ColliderHandle};
use crate::{world::World, handles::EntityId};

// Component definitions for your ECS
pub struct RigidBodyComponent {
    pub handle: BodyHandle,
    pub body_type: BodyType,
}

pub struct ColliderComponent {
    pub handle: ColliderHandle,
    pub is_sensor: bool,
}

// Physics system that can be added to your ECS
pub struct PhysicsSystem {
    world: World,
}

impl PhysicsSystem {
    pub fn new(gravity: Vector3) -> Self {
        Self {
            world: World::new(gravity),
        }
    }
    
    pub fn update(&mut self, delta_time: f32) {
        self.world.step(delta_time);
    }
    
    // Method to sync transforms with your ECS
    // The exact implementation depends on your ECS architecture
    pub fn sync_transforms(&self) -> Vec<(EntityId, Vector3, Quaternion)> {
        self.world.synchronize_transforms()
    }
    
    // Methods to add physics components to entities
    pub fn add_rigid_body(&mut self, entity: EntityId, def: &Body) -> BodyHandle {
        self.world.add_rigid_body(entity, def)
    }
    
    pub fn add_collider(&mut self, entity: EntityId, body_handle: BodyHandle, def: &ColliderDef) -> ColliderHandle {
        self.world.add_collider(entity, body_handle, def)
    }
    
}