use std::{fs, thread};
use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::Write;
use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::ptr::hash;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Instant;
use rayon::prelude::*;
use clap::{arg, ArgMatches, Command, Parser, Subcommand};
use serde::{Serialize};


fn cli() -> Command {
    Command::new("oxidesearch")
        .about("A simple search engine")
        .subcommand_required(true)
        .subcommand(
            Command::new("index")
                .about("Index the current files")
                .args(vec![arg!(-f --file <DIR_PATH>), arg!(-o --output <OUT_PATH>)])
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




#[derive(Debug, Clone, Copy, Hash, Serialize)]
struct Token {
    pub val: char,
    pub index: usize,
    // pub file: &'static str
}


fn search(file_contents: Vec<char>, haystack: Vec<Token>, needle: String) -> usize {
    fn bin_search(full_haystack: Vec<char>, haystack: &[Token], needle: String) -> Option<Token> {
        if haystack.len() == 0 {
            return None;
        }
        if haystack[0].val == needle.chars().nth(0).unwrap() {
            for (index, character) in needle.char_indices() {
                if character != full_haystack[haystack[0].index + index] {
                    return bin_search(full_haystack, &haystack[1..haystack.len() - 1], needle);
                }
            }
            return Some(haystack[0]);
        }
        let mid = haystack.len() / 2;
        if haystack[mid].val > needle.chars().nth(0).unwrap() {
            return bin_search(full_haystack, &haystack[0..mid], needle);
        } else {
            return bin_search(full_haystack, &haystack[mid..haystack.len() - 1], needle);
        }
    }
    let result = bin_search(file_contents, &haystack[0..haystack.len() - 1], needle);
    println!("{:?}", result.unwrap().index);
    result.unwrap().index
}

fn index_files(dir_path: Option<&String>, out_path: Option<&String>) {
    let path;
    if let Some(dir) = dir_path {
        path = dir.to_owned();
    } else {
        path = "./files".to_string();
    }
    let files = fs::read_dir(path).expect("Could not find the files dir!");
    let mut index_values: Vec<Token> = Vec::new();
    let mut file_contents: Vec<char> = Vec::new();
    for filePath in files {
        let file_path_result = filePath.unwrap().path();
        let file = fs::read_to_string(&file_path_result).unwrap();
        for (index, character) in file.char_indices() {
            file_contents.push(character.clone());
            index_values.push(Token {
                val: character,
                index,
                // file: file_path_result.to_str().unwrap(),
            });
        }
    }
    // let mut indexed_no_duplicates = Vec::new();
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
    let mut part_merged = slices.lock().unwrap().to_vec();
    let merged = merge_remainders(&mut slices.lock().unwrap());

    search(file_contents, merged, "epic secret that noone shall find".to_string());

    // let output_path;
    // if let Some(out) = out_path {
    //     output_path = out.to_owned();
    // } else {
    //     output_path = "./indeces/".to_string();
    // }
    //
    // if fs::read_dir(&output_path).is_err() {
    //     fs::create_dir(&output_path).unwrap();
    // }
    // let mut hasher = DefaultHasher::new();
    // fs::write(format!("./{}/index-{:?}", &output_path, merged.hash(&mut hasher)), serde_json::to_string(&merged).expect("Failed to serialize tokens")).expect("Failed to output to file!");
    // hasher.finish();
}

fn initialise() {

}







fn main() {
    let cli_args = cli().get_matches();


    //TODO: Create an init command and actually make it search!



    match cli_args.subcommand() {
        Some(("index", sub_matches)) => {
            index_files(sub_matches.get_one::<String>("file"), sub_matches.get_one::<String>("output"));
        },
        _ => {

        }
    }
}