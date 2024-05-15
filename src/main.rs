use core::{configuration::{create_configuration, Configuration}, core::{Chunk, ChunkError, ReadChunk}, format::print_chunk};
use std::{fs::File, io::Read, str};

pub mod core;

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
