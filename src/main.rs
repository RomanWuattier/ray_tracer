extern crate image;

pub mod vector;
pub mod scene;
pub mod point;
mod rendering;
mod io;

use image::{DynamicImage};
use scene::{Scene, Color, Sphere};
use point::Point;

fn build_default_scene() -> Scene {
    Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        spheres: vec![
            Sphere {
                center: Point {
                    x: 0.0,
                    y: 0.0,
                    z: -5.0,
                },
                radius: 1.0,
                color: Color {
                    red: 0.4,
                    green: 1.0,
                    blue: 0.4,
                },
            },
            Sphere {
                center: Point {
                    x: 2.0,
                    y: 1.5,
                    z:-10.0,
                },
                radius: 2.0,
                color: Color {
                    red: 1.0,
                    green: 0.5,
                    blue: 0.0,
                },
            },
            Sphere {
                center: Point {
                    x: -4.0,
                    y: 1.0,
                    z:-6.0,
                },
                radius: 3.0,
                color: Color {
                    red: 1.0,
                    green: 0.0,
                    blue: 0.0,
                },
            }
        ],
    }
}

fn ray_tracer() {
    let scene = build_default_scene();
    let img: DynamicImage = rendering::render(&scene);
    io::save(&img, "output/out.png");
}

pub fn main() {
    ray_tracer()
}
