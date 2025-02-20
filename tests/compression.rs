use layer8_primitives::compression;

#[test]
fn test_compression() {
    let laugh = "Ha".repeat(1000);
    let compressed = compression::compress_data_gzip(laugh.as_bytes()).unwrap();
    // the compressed data should be smaller than the original data
    assert!(compressed.len() < laugh.len());
    // We expect greater compression for a repeating pattern. > 50% compression
    assert!((laugh.len() as f64 * 0.5) > compressed.len() as f64);

    // lets decompress the data and check if it is the same as the original
    let decompressed = compression::decompress_data_gzip(&compressed).unwrap();
    assert_eq!(laugh.as_bytes(), decompressed.as_slice());
}

#[test]
fn test_compression_with_codec_b64() {
    let laugh = "Ha".repeat(1000);

    let compressed = compression::compress_gzip_and_encode_b64(laugh.as_bytes()).unwrap();
    // the compressed data should be smaller than the original data
    assert!(compressed.len() < laugh.len());
    // We expect greater compression for a repeating pattern
    assert!((laugh.len() as f64 * 0.5) > compressed.len() as f64);

    // lets decompress the data and check if it is the same as the original
    let decompressed = compression::decode_b64_and_decompress_gzip(&compressed).unwrap();
    assert_eq!(laugh.as_bytes(), decompressed.as_slice());
}
