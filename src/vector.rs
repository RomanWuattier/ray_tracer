use std::f64;
use std::ops::Mul;

#[derive(Copy, Clone)]
pub struct RayVector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl RayVector {
    pub fn init_zero() -> RayVector {
        RayVector::init(0.0)
    }

    pub fn init(data: f64) -> RayVector {
        RayVector { x: data, y: data, z: data }
    }

    pub fn norm(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn lenght(&self) -> f64 {
        self.norm().sqrt()
    }

    pub fn normalize(&self) -> RayVector {
        RayVector {
            x: self.x / self.lenght(),
            y: self.y / self.lenght(),
            z: self.z / self.lenght(),
        }
    }

    pub fn dot(&self, other: &RayVector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Mul<f64> for RayVector {
    type Output = RayVector;

    fn mul(self, other: f64) -> RayVector {
        RayVector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
