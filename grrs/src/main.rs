#![allow(unused)]

use clap::Parser;

/// search for a pattern...
#[derive(Parser)]
///#[command(pattern, path)]
struct Cli {
    /// pattern we are looking for
    pattern: String,
    /// file path to read
    path: std::path::PathBuf,
}

fn main() {
    ////println!("Hello, world!");
    let args = Cli::parse();
    println!("Variable 1 : {0} ", args.pattern);  // ok 
    // println!("Variable 1 & 2: {0} {1} ", args.pattern , args.path); not ok

    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    for line in content.lines() {
       if line.contains(&args.pattern) {
          println!("{}", line);
       }
    }
}
