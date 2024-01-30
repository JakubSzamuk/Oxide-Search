use std::{fs, thread};
use std::path::PathBuf;
use std::sync::mpsc;

fn merge_sort(arr: Vec<Token>) -> Vec<Token> {
    if arr.len() <= 1 {return arr};
    fn merge(mut arr1: Vec<Token>, mut arr2: Vec<Token>) -> Vec<Token> {
        let mut merged = Vec::new();

        while arr1.len() > 0 && arr2.len() > 0 {
            if arr1[0].val as u8 > arr2[0].val as u8 {
                merged.push(arr2.remove(0));
            } else {
                merged.push(arr1.remove(0));
            }
        }

        [merged, arr1, arr2].concat()
    }


    let handle_one = thread::spawn(move || {
        merge_sort(arr[0..((arr.len() as f32) / 2.).floor() as usize].to_vec())
    });
    let handle_two = thread::spawn(move || {
        merge_sort(arr[((arr.len() as f32) / 2.).floor() as usize..arr.len()].to_vec())
    });

    merge(handle_one.join().unwrap(), handle_two.join().unwrap())
}





#[derive(Debug, Clone, Copy)]
struct Token {
    pub val: char,
    pub index: usize,
    pub file: &'static str
}

fn index_files() {
    let files = fs::read_dir("./files").unwrap();
    let mut index_values = Vec::new();
    for filePath in files {
        let file_path_result = filePath.unwrap().path();
        let file = fs::read_to_string(&file_path_result).unwrap();
        for (index, character) in file.char_indices() {
            index_values.push(Token {
                val: character,
                index,
                file: file_path_result.to_str().unwrap(),
            });
        }
    }
    // let mut indexed_no_duplicates = Vec::new();

    let indexed = merge_sort(index_values);
    println!("{:?}", indexed);



}

// Index files with tokens, {val: 'h', index: 0}, then search query use binary search to find starting characters and then narrow down by adding to their index to match the rest of the search term.


fn main() {
    index_files();
}

