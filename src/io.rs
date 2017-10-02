use std::fs::File;
use std::path::Path;
use image::{DynamicImage, PNG};

pub fn save(img: &DynamicImage, path: &str) {
    let ref mut fout = File::create(&Path::new(path)).unwrap();
    let _ = img.save(fout, PNG)
        .ok()
        .expect("Saving filed");
}
