use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    
    pub fn zeros() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }
    
    pub fn ones() -> Self {
        Self { x: 1.0, y: 1.0, z: 1.0 }
    }
    
    pub fn unit_x() -> Self {
        Self { x: 1.0, y: 0.0, z: 0.0 }
    }
    
    pub fn unit_y() -> Self {
        Self { x: 0.0, y: 1.0, z: 0.0 }
    }
    
    pub fn unit_z() -> Self {
        Self { x: 0.0, y: 0.0, z: 1.0 }
    }
    
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    
    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len > 0.0 {
            Self {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        } else {
            *self
        }
    }
    
    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Add for Vector3 {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
