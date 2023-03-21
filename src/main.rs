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
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::{BufReader, Read};

    fn to_bytes(input: &Vec<u32>) -> Vec<u8> {
        let mut output = Vec::new();
        for value in input {
            output.extend(&value.to_be_bytes());
        }
        output
    }

    fn to_u32(input: &Vec<u8>) -> Vec<u32> {
        let mut output = Vec::new();
        for i in (0..input.len()).step_by(4) {
            let mut bytes = [0; 4];
            bytes.copy_from_slice(&input[i..i + 4]);
            output.push(u32::from_be_bytes(bytes));
        }
        output
    }

    #[test]
    fn test_compress_base1() {
        let data = "TOBEORNOTTOBEORTOBEORNOT";
        let data = data.as_bytes();
        let compressed = compress(data);
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
        let decompressed = decompress(&compressed);
        let decompressed = String::from_utf8(decompressed).unwrap();
        assert_eq!(decompressed, "TOBEORNOTTOBEORTOBEORNOT");
    }

    #[test]
    fn test_base2() {
        let data = "itty bitty bit bin";
        let compressed = compress(data.as_bytes());
        let decompressed = String::from_utf8(decompress(&compressed)).unwrap();
        assert!(decompressed == data);
    }

    #[test]
    fn test_simple_file() {
        let file = File::open("res/simple.txt").unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut data = String::new();
        buf_reader.read_to_string(&mut data).unwrap();

        let mut compressed = compress(data.as_bytes());

        let mut compressed_file = File::create("res/simple.lzw").unwrap();
        compressed_file
            .write_all(&to_bytes(&compressed)[..])
            .unwrap();

        let compressed_file = File::open("res/simple.lzw").unwrap();
        buf_reader = BufReader::new(compressed_file);
        let mut byte = Vec::new();
        buf_reader.read_to_end(&mut byte).unwrap();
        compressed = to_u32(&byte);

        let decompressed = String::from_utf8(decompress(&compressed)).unwrap();
        assert!(decompressed == data);
    }

    #[test]
    fn test_big_file() {
        let file = File::open("res/alice29.txt").unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut data = String::new();
        buf_reader.read_to_string(&mut data).unwrap();

        let mut compressed = compress(data.as_bytes());

        let mut compressed_file = File::create("res/alice29.lzw").unwrap();
        compressed_file
            .write_all(&to_bytes(&compressed)[..])
            .unwrap();

        let compressed_file = File::open("res/alice29.lzw").unwrap();
        buf_reader = BufReader::new(compressed_file);
        let mut byte = Vec::new();
        buf_reader.read_to_end(&mut byte).unwrap();
        let compressed_file = to_u32(&byte);
        assert!(compressed_file == compressed);

        let decompressed = String::from_utf8(decompress(&compressed)).unwrap();
        assert!(decompressed == data);
    }
}
