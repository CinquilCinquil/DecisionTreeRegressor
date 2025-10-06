use image::GenericImageView;
use crate::types::{Datapoint, rgb_datapoint};

pub fn image_to_pixels(filepath : &str) -> Vec<Datapoint> {
    let img = image::open(filepath).unwrap();
    let pixels = img.pixels();

    let mut vec : Vec<Datapoint> = vec![];
    
    for pixel in pixels {
        let color = pixel.2.0;
        let pos = (pixel.0, pixel.1);
        vec.push(rgb_datapoint(
            color[0] as i32, color[1] as i32, color[2] as i32,
            pos.0 as i32, pos.1 as i32));
    }

    println!("collected all pixels");

    return vec;
}