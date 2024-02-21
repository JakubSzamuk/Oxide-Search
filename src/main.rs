use std::{fs, thread};
use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use rayon::prelude::*;
use clap::{arg, ArgMatches, Command, Parser, Subcommand};


fn cli() -> Command {
    Command::new("oxidesearch")
        .about("A simple search engine")
        .subcommand_required(true)
        .subcommand(
            Command::new("index")
                .about("Index the current files")
                .args(vec![arg!(-f --file <DIR_PATH>)])
        )
}


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
fn merge_sort(arr: &Vec<Token>) -> Vec<Token> {
    if arr.len() <= 1 {return arr.to_vec()};

    merge(merge_sort(&arr[0..((arr.len() as f32) / 2.).floor() as usize].to_vec()), merge_sort(&arr[((arr.len() as f32) / 2.).floor() as usize..arr.len()].to_vec()).to_vec())
}

fn merge_remainders(arr: &mut Vec<Vec<Token>>) -> Vec<Token> {
    fn merge_iterator(arr1: &mut Vec<Vec<Token>>) -> Vec<Vec<Token>> {
        let arr1_len = arr1.len();
        if arr1_len == 2 {return arr1.to_vec()};
        return merge_iterator(&mut [[merge(arr1[0].clone(), arr1[1].clone())].as_mut_slice(), &mut arr1[2..arr1_len]].concat());
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

fn index_files(dir_path: Option<&String>) {
    let path;
    if let Some(dir) = dir_path {
        path = dir.to_owned();
    } else {
        path = "./files".to_string();
    }
    let files = fs::read_dir(path).expect("Could not find a files dir!");
    let mut index_values: Vec<Token> = Vec::new();
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
    let time = std::time::Instant::now();
    let cores = thread::available_parallelism().unwrap();
    let parts_per_chunk = index_values.len() / cores.get();

    let mut slices = Arc::new(Mutex::new(Vec::new()));
    index_values.par_chunks_mut(parts_per_chunk).for_each({
        slices = slices.clone();
        |slice: &mut [Token]| {
            let sorted_slice = merge_sort(&slice.to_vec()).as_slice().to_owned();
            slices.lock().unwrap().push(sorted_slice);
        }
    });
    let merged = slices.lock().unwrap().to_vec();
    // let merged = merge_remainders(&mut slices.lock().unwrap());
    // let merged = merge_sort(&index_values);
    println!("almost finished, {:?}             time: {}", merged, time.elapsed().as_secs_f64());

    // Make threads here
}

// Index files with tokens, {val: 'h', index: 0}, then search query use binary search to find starting characters and then narrow down by adding to their index to match the rest of the search term.

fn initialise() {

}


fn main() {
    let cli_args = cli().get_matches();


    //TODO: Create an init command and actually make it search!



    match cli_args.subcommand() {
        Some(("index", sub_matches)) => {
            index_files(sub_matches.get_one::<String>("file"));
        },
        _ => {}
    }
}