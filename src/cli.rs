use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, StructOpt)]
pub struct Cli {
    /// The path to the file image to read
    #[structopt(parse(from_os_str))]
    pub image_path: std::path::PathBuf,
    #[structopt(parse(from_os_str))]
    pub output_path: Option<std::path::PathBuf>,
}

