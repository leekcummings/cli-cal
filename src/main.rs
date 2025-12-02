mod args;

// Use clap for CLI argument parsing
use clap::Parser;
use chrono::prelude::*;
// Use dateparser for chrono datetime parsing
use dateparser::{parse_with};
use regex::Regex;
use std::io;

use crate::args::{CalArgs, EventOptions};

/// Default time for calendar events set to 02:00:00.000...
// Hopefully no one will be awake adding to their calendar at that millisecond
const DEFAULT_TIME: NaiveTime = NaiveTime::from_hms_opt(1, 59, 59).unwrap();

// struct CalendarEvent {
//     title: String,
//     start_date: DateTime<Local>,
//     end_date: DateTime<Local>,
//     location: Option<String>,
//     repeat: Option<String>,
//     description: Option<String>
// }

fn main() -> Result<(), String> {
    let cli = CalArgs::parse();
    match &cli.event {
        EventOptions::Add(args) => {
            let joined_dates: String = args.datetimes.join(" "); 
            let mut dates: Vec<DateTime<Local>> = Vec::new();
            let re: Regex = Regex::new(r"^(?:(.+[^\s])\s*,\s*(.+)|((.+[^,^\s])))$").unwrap();
            // I just learned how to declare variables in an if statement, is it obvious?
            if let Some(caps) = re.captures(&joined_dates) {
                for i in 1..=3 {
                    if let Some(m) = caps.get(i) && let Ok(d) = parse_with(m.as_str(), &Local, DEFAULT_TIME){
                        // Not sure why but it won't work unless I add the with_timezone()
                        dates.push(d.with_timezone(&Local));
                    }
                }
            }
            // If no dates are found in CLI input, exit
            if dates.is_empty() {
                let err: String = format!("No date(s) found in string `{joined_dates}`");
                Err(err)
            } else {
                // Print a bunch of info to confirm that the event is correct
                println!("=== Event Details ===\nTitle: {}", &args.title);
                println!("Start Date: {}", dates[0]);
                if dates.len() > 1 {
                    println!("Start Date: {}", dates[1]);
                } else {
                    println!("Start Date: {}", dates[0]);
                }
                println!("Location: {}", &args.location.clone().unwrap_or("None".to_string()));
                println!("Repeating: {}", &args.repeat.clone().unwrap_or("None".to_string()));
                println!("Description: {}", &args.description.clone().unwrap_or("None".to_string()));
                // I/O based on https://www.geeksforgeeks.org/rust/standard-i-o-in-rust/
                println!("Would you like to add this event to your calendar? Y/N");
                let mut guess = String::new();
                io::stdin().read_line(&mut guess).expect("Failed to readline");
                if guess.to_lowercase() == "y" {
                    // add to csv file
                }
                Ok(())
            } 
        }
        _ => {Ok(())}
    }
}
