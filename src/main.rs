mod args;

// Use clap for CLI argument parsing
use clap::Parser;
use crate::args::{AddArgs, CalArgs, EventOptions};

// Use dateparser for chrono datetime parsing
use chrono::prelude::*;
use dateparser::{parse, parse_with};

/// User's local timezone
const TZ: &Local = &Local;
/// Default time for calendar events set to 02:00:00.000...
// Hopefully no one will be awake adding to their calendar at that millisecond
const DEFAULT_TIME: NaiveTime = NaiveTime::from_hms_opt(1, 59, 59).unwrap();

fn main() {
    let cli = CalArgs::parse();
    match &cli.event {
        EventOptions::Add(args) => {
            println!("{:?}", args);
            println!("{:?}", parse(&args.date.start_datetime).unwrap().with_timezone(TZ));
            println!("{:?}", parse_with(&args.date.start_datetime, &Local, DEFAULT_TIME).unwrap().with_timezone(TZ));
        }
        _ => {}
    }
}
