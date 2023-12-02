// Import external packages
extern crate flate2;

// Declare specifics you want from flate2
// https://docs.rs/flate2/latest/flate2/

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;

// Accept the name of the file to be compressed
use std::fs::File;

// Compression action does copying of files
use std::io::copy;

// Before compressing or copying, read the file 
use std::io::BufReader;

use std::process::Output;
// Time taken to compress the file
use std::time::Instant;

fn main(){

    // Checks if the number of command-line arguments is not equal to 3, and if so, it prints a usage message and exits the program.
    // To execute the program, the "cargo run" command itself is considered the first argument, followed by "source" (the second argument) and "target" file name (the third argument)
    if args().len() !=3 {
        eprintln!("Enter: `source` `target");
        return;
    }

    // Open the source file to be compressed
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());

    // Create the target file where the compressed data will be written
    let output = File::create(args().nth(2).unwrap()).unwrap();
    
    // Create a GzEncoder that compresses the data and writes it to the output file
    let mut encoder = GzEncoder::new(output, Compression::default());

    // Record the current time to measure how long the compression takes
    let start = Instant::now();

    // Copy data from the input (source) file to the encoder (target)
    copy(&mut input, &mut encoder).unwrap();

    // Finish the compression process and get the compressed output
    let output = encoder.finish().unwrap();

    println!(
        "Size of source file: {:?}",
        input.get_ref().metadata().unwrap().len()
    );
    println!("Size of Compressed File: {:?}", output.metadata().unwrap().len());
    println!("Time Taken for Compression: {:?}", start.elapsed());
}