pub fn decode_gz(data: &[u8]) -> Vec<u8> {
    let mut decoder = zune_inflate::DeflateDecoder::new_with_options(
        data,
        zune_inflate::DeflateOptions::default()
            .set_confirm_checksum(true)
            .set_limit(byte_unit::n_gb_bytes!(2) as usize),
    );
    
    decoder.decode_gzip().unwrap()
}
