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

use structopt::StructOpt;

/// Transforms a picture to a pixel art
#[derive(Debug, StructOpt)]
pub struct Cli {
    /// Path to the picture to transform
    #[structopt(parse(from_os_str))]
    pub image_path: std::path::PathBuf,

    /// Path to the output file
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    pub output_path: Option<std::path::PathBuf>,

    /// Width of the output, if none width is the same as the input picture
    #[structopt(short = "W", long = "width", parse(try_from_str))]
    pub output_width: Option<u32>,

    /// Height of the output, if none height is the same as the input picture
    #[structopt(short = "H", long = "height", parse(try_from_str))]
    pub output_height: Option<u32>,
}

