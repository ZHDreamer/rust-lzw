use lzw::compress;
use lzw::decompress;

fn main() {
    let data = "";
    let data = data.as_bytes();

    let compressed = compress(data);

    println!("Compressed: {:?}", compressed);

    let decompressed = decompress(&compressed);
    let decompressed = String::from_utf8(decompressed).unwrap();

    println!("Decompressed: {}", decompressed);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_compress_base1() {
        let data = "TOBEORNOTTOBEORTOBEORNOT";
        let data = data.as_bytes();
        let compressed = super::compress(data);
        assert_eq!(
            compressed,
            vec![84, 79, 66, 69, 79, 82, 78, 79, 84, 256, 258, 260, 265, 259, 261, 263]
        );
    }

    #[test]
    fn test_decompress_base1() {
        let compressed = vec![
            84, 79, 66, 69, 79, 82, 78, 79, 84, 256, 258, 260, 265, 259, 261, 263,
        ];
        let decompressed = super::decompress(&compressed);
        let decompressed = String::from_utf8(decompressed).unwrap();
        assert_eq!(decompressed, "TOBEORNOTTOBEORTOBEORNOT");
    }
}
