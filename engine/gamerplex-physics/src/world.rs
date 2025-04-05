use std::collections::HashMap;
use std::fmt::Error;

use crate::body::*;
use crate::collider::*;
use crate::events::*;
use crate::handles::*;

use gamerplex_math::{Vector3, Quaternion};
use crate::integration::rapier;
use rapier3d::prelude::{
   RigidBodySet, 
   ColliderSet, 
   IntegrationParameters,
   PhysicsPipeline,
   IslandManager,
   BroadPhase,
   NarrowPhase,
   ImpulseJointSet,
   MultibodyJointSet,
   CCDSolver,
   QueryPipeline,
   ChannelEventCollector,
   DefaultBroadPhase,
   RigidBodyHandle
};

// the physical world struct 
pub struct  World {
   // Rapier physics objects
   rigid_body_set: RigidBodySet,
   collider_set: ColliderSet,
   integration_parameters: IntegrationParameters,
   physics_pipeline: PhysicsPipeline,
   island_manager: IslandManager,
   broad_phase: DefaultBroadPhase,
   narrow_phase: NarrowPhase,
   impulse_joint_set: ImpulseJointSet,
   multibody_joint_set: MultibodyJointSet,
   ccd_solver: CCDSolver,
   query_pipeline: QueryPipeline,

   gravity: Vector3,
   simulation_rate: f32,

   // tracking
   pub entity_body_map: HashMap<EntityId, BodyHandle>, // create BodyHandle and EntityId in handle.rs
   body_entity_map: HashMap<RigidBodyHandle, EntityId>,
   entity_collider_map: HashMap<EntityId, Vec<ColliderHandle>>,

   //collision event collection
   pub collision_events: Vec<CollisionEvent>,
   event_handler: ChannelEventCollector,
   
   //time tracking
   accumulated_time: f32,
}

impl World {
   pub fn new(gravity: Vector3) -> Self {
      // channels for both event types
      let (collision_send, collision_recv) = crossbeam::channel::unbounded();
      let (contact_force_send, contact_force_recv) = crossbeam::channel::unbounded();

      //event collector with both senders
      let event_handler = ChannelEventCollector::new(collision_send, contact_force_send);
      
      Self {
         rigid_body_set: RigidBodySet::new(),
         collider_set: ColliderSet::new(),
         integration_parameters: IntegrationParameters::default(),
         physics_pipeline: PhysicsPipeline::new(),
         island_manager: IslandManager::new(),
         broad_phase: DefaultBroadPhase::new(),
         narrow_phase: NarrowPhase::new(),
         impulse_joint_set: ImpulseJointSet::new(),
         multibody_joint_set: MultibodyJointSet::new(),
         ccd_solver: CCDSolver::new(),
         query_pipeline: QueryPipeline::new(),

         gravity: gravity,
         simulation_rate: 1.0 / 60.0, // 60 Hz 
         
         collision_events: Vec::new(),
         event_handler,
         
         entity_body_map: HashMap::new(),
         body_entity_map: HashMap::new(),
         entity_collider_map: HashMap::new(),
         
         accumulated_time: 0.0,
     }
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
   // opdate gravity
   let gravity = rapier::to_na_vector(&self.gravity);

   // run simulation
   self.physics_pipeline.step(
      &gravity,
      &self.integration_parameters,
      &mut self.island_manager,
      &mut self.broad_phase,
      &mut self.narrow_phase,
      &mut self.rigid_body_set,
      &mut self.collider_set,
      &mut self.impulse_joint_set,
      &mut self.multibody_joint_set,
      &mut self.ccd_solver,
      None,
      &(),
      &self.event_handler,
   );

   // Process collision events
   // (implementation details depend on your event system)
  }

  pub fn synchronize_transforms(&self) -> Vec<(EntityId, Vector3, Quaternion)> {
   let mut transforms = Vec::new();
   
   for (body_handle, entity) in &self.body_entity_map {
       if let Some(body) = self.rigid_body_set.get(*body_handle) {
           let position = rapier::from_na_vector(&body.translation());
           let rotation = rapier::from_na_quaternion(&body.rotation());
           
           transforms.push((*entity, position, rotation));
       }
   }
   
   transforms
  }

  pub fn add_rigid_body(&mut self, entity: EntityId, def: &Body) -> BodyHandle {
   let body = rapier::create_rigid_body(def);
   let handle_body_entity_map = self.rigid_body_set.insert(body);
   let handle = BodyHandle::new(entity);
   
   self.entity_body_map.insert(entity, handle);
   self.body_entity_map.insert(handle_body_entity_map, entity);
   
   BodyHandle(handle.into_raw_parts().0)
  }

  pub fn add_collider(&mut self, entity: EntityId, body_handle: BodyHandle, def: &ColliderDef) -> ColliderHandle {
   let rapier_body_handle = BodyHandle::from_raw_parts(body_handle.0, 0).to_rapier_handle();
   let collider = rapier::create_collider(def);
   
   let handle = self.collider_set.insert_with_parent(
       collider,
       rapier_body_handle,
       &mut self.rigid_body_set
   );
   
   self.entity_collider_map
       .entry(entity)
       .or_insert_with(Vec::new)
       .push(ColliderHandle(handle.into_raw_parts().0));
   
   ColliderHandle(handle.into_raw_parts().0)
   }

}