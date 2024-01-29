use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
struct Token {
    pub val: char,
    pub index: usize,
    pub file: PathBuf
}

fn sort_tokens() {

}

fn index_files() {
    let files = fs::read_dir("./files").unwrap();
    let mut index_values = Vec::new();
    for filePath in files {
        let filePathResult = filePath.unwrap().path();
        let file = fs::read_to_string(&filePathResult).unwrap();
        for (index, character) in file.char_indices() {
            index_values.push(Token {
                val: character,
                index,
                file: filePathResult.clone(),
            });
        }
    }
    // let mut indexed_no_duplicates = Vec::new();

    println!("{:?}", index_values);

}

// Index files with tokens, {val: 'h', index: 0}, then search query use binary search to find starting characters and then narrow down by adding to their index to match the rest of the search term.


fn main() {
    index_files();
}

