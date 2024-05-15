pub struct ReadChunk {
    pub next_index: usize,
    pub raw: String,
    pub hex: String,
}

pub struct Chunk {
    pub index: usize,
    pub raw: String,
    pub hex: String,
}

pub enum ChunkError {
    NoData,
    Utf8,
}

