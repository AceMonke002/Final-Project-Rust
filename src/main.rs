use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::{self, BufRead};
use std::path::Path;
use rayon::prelude::*;
use std::time::Instant;


// Sequential Processing
fn sequential_processing(dir: &Path) {
    let start_time = Instant::now();
    let mut word_counts: HashMap<String, u32> = HashMap::new();

    // Iterate over each entry in the directory
    for entry in fs::read_dir(dir).expect("Directory not found") {
        // Unwrap the entry
        let entry = entry.expect("Failed to read entry");

        // Check if the entry is a file
        if entry.path().is_file() {
            // Open the file
            let file = fs::File::open(entry.path()).expect("Failed to open file");
            // Create a new buffered reader for the file
            let reader = BufReader::new(file);

            // Iterate over each line in the file
            for line in reader.lines() {
                // Unwrap the line
                let line = line.expect("Failed to read line");

                // Split the line into words and iterate over them
                for word in line.split_whitespace() {
                    // Insert the word into the HashMap if it doesn't exist, and increment its count
                    *word_counts.entry(word.to_string()).or_insert(0) += 1;
                }
            }
        }
    }
    // Calculate the total number of words
    let total_words: u32 = word_counts.values().sum();
    // Calculate the total number of different words
    let total_occurrences = word_counts.len();
    // Calculate the elapsed time
    let elapsed_time = start_time.elapsed();

    // Print the results
    println!("Sequential programming: {:?}", elapsed_time);
    println!("Total Word Count: {}", total_words);
    println!("Total Number of Occurrences: {}", total_occurrences);
}

// Task Parallelism
fn task_parallelism(dir: &Path) {
    // Start the timer
    let start = Instant::now();

    // Read the directory
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
            return;
        }
    };

    // Collect the paths of the files in the directory
    let file_paths: Vec<_> = entries
        .filter_map(|res| match res {
            Ok(entry) => Some(entry.path()),
            Err(e) => {
                eprintln!("Error reading entry: {}", e);
                None
            }
        })
        .collect();

    // Process the files in parallel, counting the words in each file
    let counts: Vec<_> = file_paths.par_iter()
        .filter_map(|path| {
            let file = match File::open(path) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Error opening file: {}", e);
                    return None;
                }
            };
            let reader = io::BufReader::new(file);
            let mut word_counts = HashMap::new();
            for line in reader.lines() {
                for word in line.unwrap_or_default().split_whitespace() {
                    *word_counts.entry(word.to_string()).or_insert(0) += 1;
                }
            }
            Some((word_counts.values().sum::<usize>(), word_counts))
        })
        .collect();

    // Combine the word counts from each file
    let (total_words, word_counts): (usize, HashMap<String, usize>) = counts
        .into_iter()
        .fold((0, HashMap::new()), |(total, mut map), (count, counts)| {
            for (word, count) in counts {
                *map.entry(word).or_insert(0) += count;
            }
            (total + count, map)
        });

    // Stop the timer
    let duration = start.elapsed();

    // Print the results
    println!("Task Parallelism: {:?}", duration);
    println!("Total Word Count: {}", total_words);
    println!("Total Number of Occurrences: {}", word_counts.len())
}

fn main() {
    let dir = Path::new("files");
    task_parallelism(dir);
    sequential_processing(dir);
}
