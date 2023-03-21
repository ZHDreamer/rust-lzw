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
/// - [可能是最通俗的Lempel-Ziv-Welch (LZW)无损压缩算法详述](https://cloud.tencent.com/developer/article/1097573)
///
pub fn compress(data: &[u8]) -> Vec<usize> {
    let mut dictionary: HashMap<Vec<u8>, usize> =
        (0..256).map(|i| (vec![i as u8], i as usize)).collect();
    let mut result: Vec<usize> = Vec::new();
    let mut p: Vec<u8> = Vec::new();

    for &c in data {
        let pc = p.iter().cloned().chain(iter::once(c)).collect();
        if dictionary.contains_key(&pc) {
            p = pc;
        } else {
            result.push(dictionary[&p]);
            dictionary.insert(pc.clone(), dictionary.len() as usize);
            p = vec![c];
        }
    }

    if !p.is_empty() {
        result.push(dictionary[&p]);
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
/// - [可能是最通俗的Lempel-Ziv-Welch (LZW)无损压缩算法详述](https://cloud.tencent.com/developer/article/1097573)
pub fn decompress(data: &[usize]) -> Vec<u8> {
    let mut dictionary: HashMap<usize, Vec<u8>> =
        (0..256).map(|i| (i as usize, vec![i as u8])).collect();
    let mut result: Vec<u8> = Vec::new();

    let mut p: Vec<u8> = dictionary[&data[0]].clone();
    result.extend(&p);

    for k in &data[1..] {
        let entry = dictionary.get(k).expect("Invalid compressed data").clone();
        result.extend(&entry);
        dictionary.insert(
            dictionary.len(),
            p.into_iter().chain(entry[0..1].iter().cloned()).collect(),
        );
        p = entry;
    }

    result
}
