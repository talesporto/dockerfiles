use rand::prelude::*;
use std::path::Path;
use std::path::PathBuf;

pub fn make_unique_random_filename(parent: &Path, extension: &str) -> PathBuf {
    loop {
        let mut result: PathBuf = parent.to_path_buf();
        let mut filename: String = make_random_filename();
        filename.push('.');
        filename.push_str(extension);
        result.push(filename);
        if !result.exists() {
            return result;
        }
    }
}

fn make_random_filename() -> String {
    let mut rng = rand::thread_rng();
    let letters: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let mut result: String = String::new();
    for _ in 0..8 {
        let rng_usize: usize = rng.gen::<usize>();
        let letter_index: usize = rng_usize % letters.len();
        result.push(letters[letter_index]);
    }
    return result;
}
