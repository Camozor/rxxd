use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Configuration {
    pub file_path: PathBuf,

    #[arg(short = 'c', long = "cols", default_value_t = 16)]
    pub cols: i32,

    #[arg(short = 'g', long = "groupsize", default_value_t = 2)]
    pub group_size: i32, // Number of octets per block in the hex part
}

pub fn create_configuration() -> Configuration {
    Configuration::parse()
}
