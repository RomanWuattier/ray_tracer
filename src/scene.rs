use image::{Rgba, Pixel};
use point::Point;
use rendering::{Ray, Intersectable};

#[derive(Copy, Clone)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba::from_channels((self.red * 255.0) as u8,
            (self.green * 255.0) as u8,
            (self.blue * 255.0) as u8,
             255 as u8)
    }
}

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
}

pub struct Intersection<'a> {
    pub distance: f64,
    pub object: &'a Sphere,
}

impl<'a> Intersection<'a> {
    pub fn new(distance: f64, object: &'a Sphere) -> Intersection<'a> {
        Intersection {
            distance: distance,
            object: object,
        }
    }
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub spheres: Vec<Sphere>,
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.spheres.iter()
            .filter_map(|s| s.intersect(ray).map(|d| Intersection::new(d, s)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}
