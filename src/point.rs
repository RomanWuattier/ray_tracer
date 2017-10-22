use std::ops::Sub;
use vector::Vector3;

#[derive(Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn init_zero() -> Point {
        Point::init(0.0)
    }

    pub fn init(data: f64) -> Point {
        Point { x: data, y: data, z: data }
    }
}

impl Sub<Point> for Point {
    type Output = Vector3;

    fn sub(self, other: Point) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
