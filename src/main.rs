extern crate image;
mod cli;

use image::{/*GenericImage, GenericImageView,*/ ImageBuffer, RgbImage};
use cli::Cli;
use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();
    println!("Args = {:?}", args);
    match args.output_path {
        None => println!("Pas de fichier se dortie"),
        Some(v) => println!("Fichier de sortie !"),
    }

    // Construct a new RGB ImageBuffer with the specified width and height.
    let img: RgbImage = ImageBuffer::new(512, 512);

    // Construct a new by repeated calls to the supplied closure.
    let mut img = ImageBuffer::from_fn(512, 512, |x, y| {
        if x % 2 == 0 {
            image::Luma([0u8])
        } else {
            image::Luma([255u8])
        }
    });

    // Obtain the image's width and height.
    let (width, height) = img.dimensions();

    // Access the pixel at coordinate (100, 100).
    let pixel = img[(100, 100)];

    // Or use the `get_pixel` method from the `GenericImage` trait.
    let pixel = *img.get_pixel(100, 100);

    // Put a pixel at coordinate (100, 100).
    img.put_pixel(100, 100, pixel);

    println!("Width = {}, height = {}, pixel 100x100 = {:?}", width, height, pixel);
    img.save("test.png").unwrap();
}
