use image::GenericImageView;
use crate::types::{Datapoint, DecisionTree, DesiredClassGet, rgb_datapoint};
use crate::decision_tree;

pub fn image_to_pixels(filepath : &str) -> (Vec<Datapoint>, u32, u32) {
    let img = image::open(filepath).unwrap();
    let pixels = img.pixels();
    let (w, h) = img.dimensions();

    let mut vec : Vec<Datapoint> = vec![];
    
    for pixel in pixels {
        let color = pixel.2.0;
        let pos = (pixel.0, pixel.1);
        vec.push(rgb_datapoint(
            color[0] as i32, color[1] as i32, color[2] as i32,
            pos.0 as i32, pos.1 as i32));
    }

    println!("collected all pixels");

    return (vec, w, h);
}

pub fn generate_image_by_prediction(
    tree : &DecisionTree, desired_class : DesiredClassGet, (w, h) : (u32, u32)) {

    let mut imgbuf = image::ImageBuffer::new(w, h);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let (r, g, b) = decision_tree::predict(tree, &rgb_datapoint(0,0,0, x as i32, y as i32), desired_class);
        *pixel = image::Rgb([r as u8, g as u8, b as u8]);
    }


    imgbuf.save("result.png").unwrap();
}