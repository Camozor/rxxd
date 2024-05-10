use std::{env, fs::File, io::Read, str};
use colored::Colorize;

struct Configuration {
    file_path: String,
    cols: i32,
    group_size: i32, // Number of octets per block in the hex part
}

struct ReadChunk {
    next_index: usize,
    raw: String,
    hex: String,
}

struct Chunk {
    index: usize,
    raw: String,
    hex: String,
}

enum ChunkError {
    NoData,
    Utf8,
}

fn main() {
    let configuration = create_configuration();

    let file = File::open(configuration.file_path.clone());
    let mut file = match file {
        Ok(file) => file,
        Err(error) => panic!("No file {}", error),
    };

    let mut index = 0;
    while let Ok(read_chunk) = read_file_chunk(&mut file, index, &configuration) {
        let current_chunk = Chunk {
            index,
            raw: read_chunk.raw,
            hex: read_chunk.hex,
        };
        print_chunk(&current_chunk, &configuration);
        index = read_chunk.next_index;
    }
}

fn create_configuration() -> Configuration {
    let envs: Vec<String> = env::args().collect();

    if envs.len() <= 1 {
        panic!("Provide at least a file path");
    }
    Configuration {
        file_path: envs[1].clone(),
        cols: 16,
        group_size: 2,
    }
}

fn read_file_chunk(
    file: &mut File,
    index: usize,
    configuration: &Configuration,
) -> Result<ReadChunk, ChunkError> {
    let mut v: Vec<u8> = vec![0; configuration.cols.try_into().unwrap()];
    let read = file.read(&mut v);
    if let Ok(n) = read {
        if n == 0 {
            return Err(ChunkError::NoData);
        }
        let s = match str::from_utf8(&v) {
            Ok(str) => str,
            Err(_) => return Err(ChunkError::Utf8),
        };

        let content = s.to_string();
        let encoded = hex::encode(content.clone());

        return Ok(ReadChunk {
            next_index: index + content.len(),
            raw: content,
            hex: encoded,
        });
    }
    Err(ChunkError::NoData)
}

fn print_chunk(chunk: &Chunk, configuration: &Configuration) {
    let index = format_index(chunk);
    let hex = format_hex(chunk, configuration).green();
    let text = format_text(chunk).green();
    println!("{index} {hex} {text}");
}

fn format_index(chunk: &Chunk) -> String {
    let v = vec![chunk.index as u8];
    hex::encode(v)
}

fn format_hex(chunk: &Chunk, configuration: &Configuration) -> String {
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

fn format_text(chunk: &Chunk) -> String {
    chunk.raw.replace("\n", ".")
}
