mod args;

use chrono::prelude::*;
// Use clap for CLI argument parsing
use clap::Parser;
use csv::WriterBuilder;
// Use dateparser for chrono datetime parsing
use dateparser::{parse_with};
use regex::Regex;
use std::{fs::{self, File, OpenOptions}, io};

use crate::args::{CalArgs, EventOptions};

/// Default time for calendar events set to 02:00:00.000...
// Hopefully no one will be awake adding to their calendar at that millisecond
const DEFAULT_TIME: NaiveTime = NaiveTime::from_hms_opt(1, 59, 59).unwrap();

// Object to be appended to CSV
#[derive(serde::Serialize)]
struct CalendarEvent {
    title: String,
    start_date: String,
    end_date: String,
    location: String,
    repeat: String,
    description: String
}

fn main() -> Result<(), String> {
    let cli = CalArgs::parse();
    // Switch case for each of the available CLI options
    match &cli.event {
        // ADD TO CALENDAR
        EventOptions::Add(args) => {
            // Parsing String into DateTimes
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
            // End of parsing DateTimes
            // If no dates are found in CLI input, exit
            if dates.is_empty() {
                let err: String = format!("No date(s) found in string `{joined_dates}`");
                Err(err)
            } else {
                // Does the event end of the same day or on a different day?
                let mut end: String = String::new();
                // Different Day
                if dates.len() > 1 {
                    end = dates[1].to_string();
                // Same Day
                } else {
                    end = dates[0].to_string();
                }
                // Create calendar event to add to csv
                let e: CalendarEvent = CalendarEvent {
                    title: args.title.clone(),
                    start_date: dates[0].to_string(),
                    end_date: end.clone(),
                    location: args.location.clone().unwrap_or("None".to_string()),
                    repeat: args.repeat.clone().unwrap_or("None".to_string()),
                    description: args.description.clone().unwrap_or("None".to_string())
                };
                // Print a bunch of info to confirm that the event is correct
                println!("=== Event Details ===\nTitle: {}", e.title);
                println!("Start Date: {}", e.start_date);
                println!("End Date: {}", e.end_date);
                println!("Location: {}", e.location);
                println!("Repeating: {}", e.repeat);
                println!("Description: {}", e.description);
                // I/O based on https://www.geeksforgeeks.org/rust/standard-i-o-in-rust/
                println!("Would you like to add this event to your calendar? Y/N");
                let mut response = String::new();
                io::stdin().read_line(&mut response).expect("Failed to readline");

                if response.to_lowercase() == "y\n" {
                    // Check that save file exists
                    let file_name = "events.csv";
                    // If the file doesn't exist
                    if !fs::exists(file_name).unwrap() {
                        let mut file = File::create(file_name).unwrap();
                        // Big difference: Add a header to the csv
                        let mut wtr = WriterBuilder::new()
                            .has_headers(true)
                            .from_writer(file);
                        wtr.serialize(e);
                    // If the file exists
                    } else {
                        // Open file in APPEND mode to not overwrite
                        let file: File = OpenOptions::new().append(true).open(file_name).unwrap();
                        let mut wtr = WriterBuilder::new()
                            .has_headers(false)
                            .from_writer(file);
                        wtr.serialize(e);
                    }
                }
                // Everything worked!
                Ok(())
            } 
        }
        _ => {Ok(())}
    }
}
