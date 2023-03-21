use std::collections::HashMap;
use std::iter;

/// Compresses a vector of data.
///
/// # Arguments
///
/// - `data` - A array of uncompressed data in **byte**.
///
/// # Returns
///
/// A vector of compressed data.
///
/// # Examples
///
/// ``` rust
/// use lzw::compress;
/// let data = "TOBEORNOTTOBEORTOBEORNOT";
/// let data = data.as_bytes();
/// let compressed = compress(data);
/// assert_eq!(compressed, vec![84, 79, 66, 69, 79, 82, 78, 79, 84, 256, 258, 260, 265, 259, 261, 263]);
/// ```
///
/// # References
///
/// - [Lempel–Ziv–Welch](https://en.wikipedia.org/wiki/Lempel%E2%80%93Ziv%E2%80%93Welch)
///

pub fn compress(data: &[u8]) -> Vec<u32> {
    if data.is_empty() {
        return Vec::new();
    }

    let mut dictionary: HashMap<Vec<u8>, u32> =
        (0..256).map(|i| (vec![i as u8], i as u32)).collect();
    let mut result: Vec<u32> = Vec::new();

    let mut p: Vec<u8> = vec![data[0]];

    for &c in &data[1..] {
        let pc = p.iter().cloned().chain(iter::once(c)).collect();
        if dictionary.contains_key(&pc) {
            p = pc;
        } else {
            dictionary.insert(pc.clone(), dictionary.len() as u32);
            result.push(dictionary[&p] as u32);
            p = vec![c];
        }
    }

    if !p.is_empty() {
        result.push(dictionary[&p] as u32);
    }

    result
}

/// Decompresses a vector of compressed data.
///
/// # Arguments
///
/// - `data` - A array of compressed data.
///
/// # Returns
///
/// A vector of decompressed data.
///
/// # Panics
///
/// Panics if the compressed data is invalid.
///
/// # Examples
///
/// ``` rust
/// use lzw::decompress;
/// let compressed = vec![84, 79, 66, 69, 79, 82, 78, 79, 84, 256, 258, 260, 265, 259, 261, 263];
/// let decompressed = decompress(&compressed);
/// let decompressed = String::from_utf8(decompressed).unwrap();
/// assert_eq!(decompressed, "TOBEORNOTTOBEORTOBEORNOT");
/// ```
///
/// # References
///
/// - [Lempel–Ziv–Welch](https://en.wikipedia.org/wiki/Lempel%E2%80%93Ziv%E2%80%93Welch)
///

pub fn decompress(data: &[u32]) -> Vec<u8> {
    if data.is_empty() {
        return Vec::new();
    }

    let mut dictionary: HashMap<u32, Vec<u8>> =
        (0..256).map(|i| (i as u32, vec![i as u8])).collect();
    let mut result: Vec<u8> = Vec::new();

    let mut p: Vec<u8> = dictionary
        .get(&data[0])
        .expect("Invalid compressed data")
        .clone();
    result.extend(&p.clone());

    for &k in &data[1..] {
        p = match dictionary.get(&k) {
            Some(value) => {
                let entry = value.clone();
                result.extend(entry.clone());
                dictionary.insert(
                    dictionary.len() as u32,
                    p.into_iter().chain(entry[0..1].iter().cloned()).collect(),
                );
                entry
            }
            None => {
                p.push(p[0]);
                result.extend(p.clone());
                dictionary.insert(dictionary.len() as u32, p.clone());
                p
            }
        };
    }

    result
}

pub fn to_bytes(input: &Vec<u32>) -> Vec<u8> {
    let mut output = Vec::new();
    for value in input {
        output.extend(&value.to_be_bytes());
    }
    output
}

pub fn to_u32(input: &Vec<u8>) -> Vec<u32> {
    let mut output = Vec::new();
    for i in (0..input.len()).step_by(4) {
        let mut bytes = [0; 4];
        bytes.copy_from_slice(&input[i..i + 4]);
        output.push(u32::from_be_bytes(bytes));
    }
    output
}
