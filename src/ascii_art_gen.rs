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

use image::{RgbImage, imageops};

pub struct AsciiArtGen {
    image_original: RgbImage,
    output_width: u32,
    output_height: u32,
}

impl AsciiArtGen {
    pub fn new(img: RgbImage, o_w: u32, o_h: u32) -> AsciiArtGen {
        AsciiArtGen {
            image_original: img,
            output_width: o_w,
            output_height: o_h,
        }
    }

    pub fn generate(&self) -> String {
        let mut result: String = String::with_capacity((self.output_height * self.output_width + self.output_height) as usize);

        // Start by converting image to grayscale
        let mut image = imageops::colorops::grayscale(&self.image_original);

        // Resize to asked dimensions if necessary
        if self.output_width != self.image_original.width() || self.output_height != self.image_original.height() {
            image = imageops::resize(&image, self.output_width, self.output_height, imageops::CatmullRom);
        }

        // Blur to have more uniform color
        image = imageops::blur(&image, 2.0);

        // Generate!
        for h in 0..image.height() {
            for w in 0..image.width() {
                let p = image.get_pixel(w, h).0[0];
                if p <= 25 {
                    result.push('#');
                } else if p <= 50 {
                    result.push('@');
                } else if p <= 75 {
                    result.push('B');
                } else if p <= 100 {
                    result.push('G');
                } else if p <= 125 {
                    result.push('O');
                } else if p <= 150 {
                    result.push('D');
                } else if p <= 175 {
                    result.push('+');
                } else if p <= 200 {
                    result.push('-');
                } else if p <= 225 {
                    result.push('.');
                } else {
                    result.push(' ');
                }
            }
            result.push('\n');
        }

        result
    }
}
