# Final-Project-Rust

# Introduction
- For this final project, you'll be creating a word counting application in Rust that utilizes different parallelism techniques. This will help you understand how Rust handles concurrency and parallel workloads effectively.
- Your implementations will focus on three main versions: sequential, task parallelism, and pipeline parallelism using the actor model.

# Requirements
1. Input Data: Use ten text files, each at least 10 KB in size.
2. Functional Goal: Count the number of occurrences of each word across all files. The counts from each file must be aggregated.

# Implementation Details
1. Sequential Processing:
- Read each file one at a time and count the occurrences of each word.
- Use a HashMap to track word counts.
- This method will serve as the baseline for performance comparison.
2. Task Parallelism:
- Implement using the rayon library.
- Split the task of reading and counting words across multiple threads.
- Each thread processes one or more files and returns a partial result.
- Merge these partial results into a final result.
3. Pipeline Parallelism with Actor Model:
- Design this using a series of actors where each actor is responsible for a part of the task (e.g., reading files, counting words, aggregating counts).
- Use channels to pass data between actors.
- This approach should show how tasks can be decomposed in a more complex, but potentially more efficient manner.

# Performance Measurement
- Measure the time taken by each implementation to process the files.
- Compare these times to discuss the efficiency and effectiveness of each parallelism technique.

# Documentation and Comments
- Extensively comment your code to explain:
    - The functionality of each part.
    - The Rust-specific features used (e.g., ownership, borrowing, threads).
    - Details on the parallelism techniques applied.

# Bonus Task: Data Compression Dictionary
- Implement a data compression technique where frequent words are stored in a dictionary with shorter representations.
- This could be a Huffman coding approach or another suitable compression algorithm like Lempel-Ziv-Welch (LZW)
- Measure and discuss the effectiveness of this compression in terms of space saved.

# Submission Guidelines
- Provide a link to your Repl.it project.
- Include screenshots of your application's output.
- Ensure your code compiles and runs without errors.

# Grading Criteria
- Code Compilation: The application must compile and run.
- Correctness: Outputs must be accurate for the word counts.
- Comment Quality: Comments should be clear, concise, and informative.
- Performance Comparison: Document and analyze the performance of each version.

# Deadline
- Submission is due by the end of the day on May 8, 2024.