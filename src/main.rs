use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::time::Instant;

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

fn main() {
    let dir = Path::new("files");
    sequential_processing(dir);
}
