use std::{fs, thread};
use std::num::NonZeroUsize;
use std::thread::JoinHandle;


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
fn merge_sort(arr: Vec<Token>) -> Vec<Token> {
    if arr.len() <= 1 {return arr};

    merge(merge_sort(arr[0..((arr.len() as f32) / 2.).floor() as usize].to_vec()), merge_sort(arr[((arr.len() as f32) / 2.).floor() as usize..arr.len()].to_vec()))
}

fn merge_remainders(arr: Vec<Vec<Token>>) -> Vec<Token> {
    fn merge_iterator(arr1: Vec<Vec<Token>>) -> Vec<Vec<Token>> {
        if arr1.len() == 2 {return arr1};

        let mut result = Vec::new();
        return merge_iterator([[merge(arr1[0].clone(), arr1[1].clone())], arr1[2..arr1.len()].to_vec()].concat());
    }

    let merged = merge_iterator(arr);
    return merge(merged[0].clone(), merged[1].clone());
}




#[derive(Debug, Clone, Copy)]
struct Token {
    pub val: char,
    pub index: usize,
    // pub file: &'static str
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
                // file: file_path_result.to_str().unwrap(),
            });
        }
    }
    // let mut indexed_no_duplicates = Vec::new();

    let cores = thread::available_parallelism().unwrap();
    let sliceLength = index_values.len() / cores.get();

    let mut handlers: Vec<JoinHandle<Vec<Token>>> = Vec::with_capacity(cores.get());

    for i in 0..cores.get() {
        let threadSlice = index_values[(i - 1 as usize) * sliceLength..i*sliceLength];
        handlers[i] = thread::spawn(move || {
            merge_sort(threadSlice.to_vec())
        });
    }
    let mut results: Vec<Vec<Token>> = Vec::with_capacity(cores.get());

    handlers.iter().for_each(|handler| {
        results.push(handler.join().unwrap());
    });
    let final_indexing = merge_remainders(results);
    print!("{:?}", final_indexing);









    // Make threads here





}

// Index files with tokens, {val: 'h', index: 0}, then search query use binary search to find starting characters and then narrow down by adding to their index to match the rest of the search term.


fn main() {
    index_files();
}

