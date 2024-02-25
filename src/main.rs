extern crate flate2;

use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::io::Write;
use std::time::Instant;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 4 {
        eprintln!("Usage: <commpress|uncompress>, 'source', `target`");
        return;
    }
    let command = &args[1];
    let source = &args[2];
    let target = &args[3];

    match command.as_str() {
        "compress" => compress_file(source, target),
        "uncompress" => uncompress_file(source, target),
        _ => eprintln!("Invalid command. Use 'compress' or 'uncompress'."),
    }
}

fn compress_file(source: &str, target: &str){
    let mut input  = BufReader::new(File::open(source).unwrap());
    let output = File::create(target).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );
    println!("Target len: {:?}", output.metadata().unwrap().len());
    println!("Compressed successfully.");
    println!("Elapsed: {:?}", start.elapsed());
}

fn uncompress_file(source: &str, target: &str){
    let input = File::open(source).unwrap();
    let mut decoder = GzDecoder::new(input); // No unwrap here
    let mut output = File::create(target).unwrap();
    let start = Instant::now();
    copy(&mut decoder, &mut output).unwrap();
    output.flush().unwrap(); // Ensure all data is written to the file
    println!("Uncompressed successfully.");
    println!("Elapsed: {:?}", start.elapsed());
    println!(
        "Source len: {:?}",
        File::open(source).unwrap().metadata().unwrap().len() // Get the size of the source file
    );
    // Get the size of the uncompressed file
    println!("Target len: {:?}", File::open(target).unwrap().metadata().unwrap().len());
}