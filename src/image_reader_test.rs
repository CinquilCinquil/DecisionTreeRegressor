use image::GenericImageView;

pub fn l(filepath : &str) -> image::DynamicImage {
    return image::open(filepath).unwrap();
} // (u32, u32, image::Rgba<u8>)

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

pub fn test() {
    let img = image::open("ex2.png").unwrap();    
    println!("dimensions {:?}", img.dimensions());
    println!("{:?}", img.color());
    println!("{:?}", img.get_pixel(0, 0));
    println!("{:?}", img.get_pixel(1, 0));

    let (w, h) = img.dimensions();
    let pixels = img.pixels();

    let mut i = 0;
    for pixel in pixels {
        print_type_of(&pixel);
        let color = pixel.2.0;
        let pos = (pixel.0, pixel.1);
        println!("{:?} {:?}", pos, color);
        if i > 3 {
            break;
        }
        i += 1;
    }

    //sla.next();
    //println!("{:?}", sla);

    /*
    for i in 0..w {
        for j in 0..h {

        }
    }
    */

    //img.save("test.png").unwrap();
}

fn main() {

}