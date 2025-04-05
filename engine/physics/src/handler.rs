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
}