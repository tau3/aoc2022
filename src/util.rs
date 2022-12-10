use std::fs;
use std::path::PathBuf;

pub fn read_real_data(file: &str) -> Vec<String> {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dir.push("resources/test/");
    dir.push(file);

    let contents =
        fs::read_to_string(&dir).unwrap_or_else(|_| format!("failed to read from: {:?}", dir));
    contents.lines().map(|line| line.to_owned()).collect()
}
