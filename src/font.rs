pub fn get_babelstonehan() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let font = include_bytes!("../assets/BabelStoneHan.ttf.zst");
    return Ok(decode_zstd(font)?);
}

fn decode_zstd(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let bound = zstd_safe::decompress_bound(&data).expect("zstd_safe::decompress_bound failed");
    let mut decompressed: Vec<u8> = Vec::with_capacity(bound.try_into()?);
    zstd_safe::decompress(&mut decompressed, &data).expect("zstd_safe::decompress failed");
    return Ok(decompressed);
}