use clap::Parser;
use lzw::*;
use std::io::{self, Read, Write};
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    #[arg(
        short = 'i',
        long = "input",
        help = "Input file, if not specified, stdin will be used"
    )]
    input: Option<PathBuf>,
    #[arg(
        short = 'o',
        long = "output",
        help = "Output file, if not specified, stdout will be used"
    )]
    output: Option<PathBuf>,
    #[arg(
        short = 'd',
        long = "decompress",
        default_value = "false",
        help = "If true decompress the input file, else compress the input file"
    )]
    decompress: bool,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    let mut buffer = Vec::new();

    match args.input.clone() {
        Some(path) => {
            let file = std::fs::File::open(path).expect("File not found");
            let mut buf_reader = std::io::BufReader::new(file);
            buf_reader
                .read_to_end(&mut buffer)
                .expect("Error reading file");
        }
        None => {
            let stdin = io::stdin();
            stdin
                .lock()
                .read_to_end(&mut buffer)
                .expect("Failed to read from stdin");
        }
    }

    let output = match args.decompress {
        false => to_bytes(&compress(&buffer)),
        true => decompress(&to_u32(&buffer)),
    };

    match args.output {
        Some(path) => {
            let mut file = std::fs::File::create(path).expect("Failed to create file");
            file.write_all(&output)
        }
        None => {
            let mut stdout = io::stdout();
            stdout.write_all(&output)
            // println!("{:?}", output);
            // Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_base3() {
        let data = "AAAABBBB";
        let compressed = compress(data.as_bytes());
        let decompressed = String::from_utf8(decompress(&compressed)).unwrap();
        assert!(decompressed == data);
    }

    #[test]
    fn test_simple_file() {
        let mut buffer = Vec::new();
        let path = "res/simple.txt";
        let file = std::fs::File::open(path).expect("File not found");
        let mut buf_reader = std::io::BufReader::new(file);
        buf_reader
            .read_to_end(&mut buffer)
            .expect("Error reading file");
        let compressed = to_bytes(&compress(&buffer));
        let decompress = decompress(&to_u32(&compressed));

        assert_eq!(buffer, decompress);
    }

    #[test]
    fn test_big_file1() {
        let mut buffer = Vec::new();
        let path = "res/alice29.txt";
        let file = std::fs::File::open(path).expect("File not found");
        let mut buf_reader = std::io::BufReader::new(file);
        buf_reader
            .read_to_end(&mut buffer)
            .expect("Error reading file");
        let compressed = to_bytes(&compress(&buffer));
        let decompress = decompress(&to_u32(&compressed));

        assert_eq!(buffer, decompress);
    }

    #[test]
    fn test_big_file2() {
        let mut buffer = Vec::new();
        let path = "res/bible.txt";
        let file = std::fs::File::open(path).expect("File not found");
        let mut buf_reader = std::io::BufReader::new(file);
        buf_reader
            .read_to_end(&mut buffer)
            .expect("Error reading file");
        let compressed = to_bytes(&compress(&buffer));
        let decompress = decompress(&to_u32(&compressed));

        assert_eq!(buffer, decompress);
    }
}
