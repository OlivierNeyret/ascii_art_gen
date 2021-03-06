// Copyright (C) 2020 Olivier Neyret
//
// This file is part of ascii_art_gen.
//
// ascii_art_gen program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// ascii_art_gen program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

extern crate image;
mod cli;
mod ascii_art_gen;

use cli::Cli;
use ascii_art_gen::AsciiArtGen;
use structopt::StructOpt;
use std::fs;

fn main() {
    let args = Cli::from_args();

    let img = match image::open(args.image_path) {
        Err(e) => {
            println!("{:?}", e);
            return ();
        }
        Ok(v) => v.to_rgb(),
    };

    let output_width: u32 = match args.output_width {
        None => img.width(),
        Some(w) => w,
    };

    let output_height: u32 = match args.output_height {
        None => img.height(),
        Some(h) => h,
    };

    let gen = AsciiArtGen::new(img, output_width, output_height);
    let ascii = gen.generate();

    match args.output_path {
        None => {
            println!("{}", ascii);
        }
        Some(v) => {
            fs::write(v, ascii).expect("Unable to write to file");
        }
    }

    /*
    image::imageops::resize(&img, output_width, output_height, image::imageops::Nearest).save("nearest.jpg").unwrap();
    image::imageops::resize(&img, output_width, output_height, image::imageops::Triangle).save("triangle.jpg").unwrap();
    image::imageops::resize(&img, output_width, output_height, image::imageops::CatmullRom).save("catmull.jpg").unwrap();
    image::imageops::resize(&img, output_width, output_height, image::imageops::Gaussian).save("gaussian.jpg").unwrap();
    image::imageops::resize(&img, output_width, output_height, image::imageops::Lanczos3).save("lanczos3.jpg").unwrap();
    */

    // Create a new image with a red line
    /*
    //let img: RgbImage = img_try.unwrap().to_rgb();
    let pixel = img[(100, 100)];
    println!("Pixel = {:?}", pixel);

    let mut img2: RgbImage = ImageBuffer::new(img.width(), img.height());
    for x in 0..img.width() {
        for y in 0..img.height() {
            let new_pixel = img2.get_pixel_mut(x, y);
            if y == 50 {
                *new_pixel = image::Rgb([255, 0, 0]);
            } else {
                let color = img.get_pixel(x, y);
                *new_pixel = *color;
            }
        }
    }

    img2.save("test.jpg").unwrap();
    */

    /*

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
    */
}
