mod args;

// Use clap for CLI argument parsing
use clap::{Parser};
use crate::args::{CalArgs, EventOptions};

// Use dateparser for chrono datetime parsing

//     let re: Regex = Regex::new(r"^(?:(.+[^\s])\s*-\s*(.+)|((.+[^-^\s])))$").unwrap();


fn main() {
    let cli = CalArgs::parse();
    match &cli.event {
        EventOptions::Add(args) => {
            println!("{:?}", args);
            // Parse datetime
            // add to json file
        }
        _ => {}
    }
}
