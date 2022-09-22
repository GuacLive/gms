#![no_main]
use libfuzzer_sys::fuzz_target;
use rtmp::{
    chunk::unpacketizer::{ChunkUnpacketizer, UnpackResult},
    messages::parser::MessageParser,
};

fuzz_target!(|data: &[u8]| {
    let mut parser = ChunkUnpacketizer::new();
    parser.extend_data(data);
    if let Ok(UnpackResult::Chunks(chunks)) = parser.read_chunks() {
        for chunk in chunks {
            let _ = MessageParser::new(chunk).parse();
        }
    }
});
