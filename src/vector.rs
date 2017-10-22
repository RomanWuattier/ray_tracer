use std::f64;
use std::ops::Mul;

#[derive(Copy, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn init_zero() -> Vector3 {
        Vector3::init(0.0)
    }

    pub fn init(data: f64) -> Vector3 {
        Vector3 { x: data, y: data, z: data }
    }

    pub fn norm(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn lenght(&self) -> f64 {
        self.norm().sqrt()
    }

    pub fn normalize(&self) -> Vector3 {
        Vector3 {
            x: self.x / self.lenght(),
            y: self.y / self.lenght(),
            z: self.z / self.lenght(),
        }
    }

    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: f64) -> Vector3 {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
