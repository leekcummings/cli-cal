mod args;

// Use clap for CLI argument parsing
use clap::{Parser};
use crate::args::{AddArgs, CalArgs, EventOptions};

// Use dateparser for chrono datetime parsing
use chrono::prelude::*;
use dateparser::{parse_with};

use regex::Regex;

/// Default time for calendar events set to 02:00:00.000...
// Hopefully no one will be awake adding to their calendar at that millisecond
const DEFAULT_TIME: NaiveTime = NaiveTime::from_hms_opt(1, 59, 59).unwrap();

// FAILED CODE
// I assume I won't need this but just in case...
// fn date_parse(dates: &Vec<String>, vec: &mut Vec<String>, mut string: String) -> Vec<String> {
//     for i in dates {
//         if i.contains("-") && i == "-" {
//             vec.push(string);
//             string = String::new();
//         } else if i.contains("-") {
//             let parts: std::str::Split<'_, &str> = i.split("-");
//             let collection:&Vec<String> = &parts.map(|v| v.to_string()).collect();
//             date_parse(collection, vec, string.clone());
//         } else if string == String::new() {
//             string.push_str(&i);
//         }
//         else {
//             string.push_str(" ");
//             string.push_str(&i);
//         }
//         println!("{i}")
//     }
//     vec.push(string);
//     vec.to_vec()
// }

/// Validate CLI input for datetimes
// Need to add handling for failed datetimes
fn date_parse(args: &AddArgs) -> Vec<DateTime<Local>> {
    let joined_dates: String = args.datetimes.join(" "); 
    let re: Regex = Regex::new(r"^(?:(.+[^\s])\s*-\s*(.+)|((.+[^-^\s])))$").unwrap();
    
    let mut vec: Vec<DateTime<Local>> = Vec::new();
    // I just learned how to declare variables in if statement, is it obvious?
    if let Some(caps) = re.captures(&joined_dates) {
        for i in 1..=3 {
            if let Some(m) = caps.get(i) && let Ok(d) = parse_with(m.as_str(), &Local, DEFAULT_TIME){
                // Not sure why but it won't work unless I add the with_timezone()
                vec.push(d.with_timezone(&Local));
            }
        }
    }
    vec
}

fn main() {
    let cli = CalArgs::parse();
    match &cli.event {
        EventOptions::Add(args) => {
            println!("{:?}", date_parse(args));
        }
        _ => {}
    }
}
