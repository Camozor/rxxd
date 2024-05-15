use colored::Colorize;

use super::{configuration::Configuration, core::Chunk};

pub fn print_chunk(chunk: &Chunk, configuration: &Configuration) {
    let index = format_index(chunk);
    let hex = format_hex(chunk, configuration).green();
    let text = format_text(chunk).green();
    println!("{index} {hex} {text}");
}

pub fn format_index(chunk: &Chunk) -> String {
    let v = vec![chunk.index as u8];
    hex::encode(v)
}

pub fn format_hex(chunk: &Chunk, configuration: &Configuration) -> String {
    let mut formatted = String::from("");
    let delimiter = " ";

    let str = chunk.hex.clone();
    let max = str.len();
    let block_size = 2 * configuration.group_size as usize;

    let mut index = 0;
    loop {
        if index >= max {
            break;
        }
        let block: String = str.chars().skip(index).take(block_size).collect();
        formatted.push_str(&block);
        formatted.push_str(delimiter);
        index += block_size;
    }

    formatted
}

pub fn format_text(chunk: &Chunk) -> String {
    chunk.raw.replace("\n", ".")
}
