use std::fs;
use std::path::PathBuf;

fn mergeSort(arr: Vec<Token>) -> Vec<Token> {
    fn merge(mut arr1: Vec<Token>, mut arr2: Vec<Token>) -> Vec<Token> {
        let mut merged = Vec::new();

        while (arr1.len() > 0 && arr2.len() > 0) {
            if (arr1[0].val as u8 > arr2[0].val as u8) {
                merged.push(arr2.remove(0));
            } else {
                merged.push(arr1.remove(0));
            }
        }

        [merged, arr1, arr2].concat()
    }

    let half = ((arr.len() as i16) / 2).floor();
    let firstHalf = mergeSort(arr[0..half].to_vec());
    let secondHalf = mergeSort(arr[half..arr.len()].to_vec());

    return merge(firstHalf, secondHalf);
}













#[derive(Debug, Clone)]
struct Token {
    pub val: char,
    pub index: usize,
    pub file: PathBuf
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

    let indexed = mergeSort(index_values);
    println!("{:?}", indexed);



}

// Index files with tokens, {val: 'h', index: 0}, then search query use binary search to find starting characters and then narrow down by adding to their index to match the rest of the search term.


fn main() {
    index_files();
}

