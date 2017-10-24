extern crate image;

pub mod vector;
pub mod scene;
pub mod point;
mod rendering;
mod io;

use image::{DynamicImage, GenericImage};
use scene::Scene;

#[test]
pub fn render_scene_test() {
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        elements: vec![],
    };

    let img: DynamicImage = rendering::render(&scene);
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());
}
