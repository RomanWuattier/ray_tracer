use std::f64;
use std::option;
use image::{DynamicImage, GenericImage, Rgba, Pixel};
use scene::{Scene, Sphere};
use point::Point;
use vector::RayVector;

pub fn render(scene: &Scene) -> DynamicImage {
    let mut img = DynamicImage::new_rgb8(scene.width, scene.height);
    let black = Rgba::from_channels(0, 0, 0, 0);
    for x in 0..scene.width {
        for y in 0..scene.height {
            let prime_ray = Ray::create_prime(x, y, scene);

            if scene.sphere.intersect(&prime_ray) != None {
                img.put_pixel(x, y, scene.sphere.color.to_rgba());
            } else {
                img.put_pixel(x, y, black);
            }
        }
    }
    img
}

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: RayVector,
}

impl Ray {
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
        assert!(scene.width >= scene.height);
        let fov = (scene.fov.to_radians() / 2.0).tan();
        let aspect_ratio = scene.width as f64 / scene.height as f64;
        // Sensor square between (-1.0 ... 1.0) (-1.0 ... 1.0)
        let sensor_x = ((((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) *
                        aspect_ratio) * fov;
        let sensor_y = 1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0;

        Ray {
            origin: Point::init_zero(),
            direction: RayVector {
                x: sensor_x,
                y: sensor_y,
                z: -1.0,
            }
            .normalize()
        }
    }
}

trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        // Pythagorean theorem
        let hypotenuse: RayVector = self.center - ray.origin;
        let adj = hypotenuse.dot(&ray.direction);
        let distance2 = hypotenuse.dot(&hypotenuse) - (adj * adj);
        let radius2 = self.radius * self.radius;

        if distance2 > radius2 {
            return None;
        }

        let thickness = (radius2 - distance2).sqrt();
        let distance = adj - thickness;

        if distance < 0.0 {
            return None;
        }

        Some(distance)
    }
}
