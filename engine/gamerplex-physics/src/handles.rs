use rapier3d::prelude::RigidBodyHandle;

pub type EntityId = u32; // Or whatever your ECS uses

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BodyHandle(pub u32);

impl BodyHandle {
    pub fn new(id: u32) -> Self {
        Self(id)
    }
    
    pub fn invalid() -> Self {
        Self(u32::MAX)
    }
    
    pub fn is_valid(&self) -> bool {
        self.0 != u32::MAX
    }

    pub fn into_raw_parts(self) -> (u32, u32) {
        (self.0, 0) // The second value is the generation, which we set to 0
    }

    pub fn from_raw_parts(index: u32, _generation: u32) -> Self {
        Self(index)
    }

    pub fn to_rapier_handle(&self) -> RigidBodyHandle {
        RigidBodyHandle::from_raw_parts(self.0, 0)
    }
    
    // Create your handle from Rapier's handle
    pub fn from_rapier_handle(handle: RigidBodyHandle) -> Self {
        Self(handle.into_raw_parts().0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ColliderHandle(pub u32);

impl ColliderHandle {
    pub fn new(id: u32) -> Self {
        Self(id)
    }
    
    pub fn invalid() -> Self {
        Self(u32::MAX)
    }
    
    pub fn is_valid(&self) -> bool {
        self.0 != u32::MAX
    }

    pub fn into_raw_parts(self) -> (u32, u32) {
        (self.0, 0) 
    }

    // Static method to create from raw parts
    pub fn from_raw_parts(index: u32, _generation: u32) -> Self {
        //ignore the generation for now
        Self(index)
    }
}