use std::io::prelude::*;

use base64::{self, engine::general_purpose::URL_SAFE as base64_enc_dec, Engine as _};
use flate2::bufread::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;

pub fn compress_gzip_and_encode_b64(data: &[u8]) -> Result<String, String> {
    let compressed_data = compress_data_gzip(data)?;
    Ok(base64_enc_dec.encode(&compressed_data))
}

pub fn decode_b64_and_decompress_gzip(encoded_data: &str) -> Result<Vec<u8>, String> {
    let compressed_data = base64_enc_dec
        .decode(encoded_data)
        .map_err(|e| e.to_string())?;
    decompress_data_gzip(&compressed_data)
}

/// This function compresses the provided data using the gzip algorithm. It uses
/// a default compression level.
pub fn compress_data_gzip(chunk: &[u8]) -> Result<Vec<u8>, String> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(chunk).map_err(|e| e.to_string())?;
    encoder.finish().map_err(|e| e.to_string())
}

/// This function decompresses the provided data using the gzip algorithm.
pub fn decompress_data_gzip(compressed_chunk: &[u8]) -> Result<Vec<u8>, String> {
    let mut decoder = GzDecoder::new(compressed_chunk);
    let mut decompressed_data = Vec::new();
    _ = decoder
        .read_to_end(&mut decompressed_data)
        .map_err(|e| e.to_string())?;

    Ok(decompressed_data)
}
