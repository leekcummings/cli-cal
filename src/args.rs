// Used this video as main reference https://www.youtube.com/watch?v=fD9ptABVQbI
use clap:: {
    Args,
    Parser,
    Subcommand, builder::{Str, ValueParser}
};

use chrono::prelude::*;
use dateparser::{parse_with};

use regex::Regex;

/// Default time for calendar events set to 02:00:00.000...
// Hopefully no one will be awake adding to their calendar at that millisecond
const DEFAULT_TIME: NaiveTime = NaiveTime::from_hms_opt(1, 59, 59).unwrap();

fn date_parse(raw_date: &str) -> Result<chrono::DateTime<Utc>, anyhow::Error> {
    println!("{}", &raw_date);
    parse_with(raw_date, &Utc, DEFAULT_TIME)
}

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct CalArgs {
    #[clap(subcommand)]
    pub event: EventOptions
}

#[derive(Debug, Subcommand)]
pub enum EventOptions {
    /// Add event to your calendar
    Add(AddArgs),
    /// Edit an existing event in calendar
    // Use wild args to find name using regex
    // https://crates.io/crates/wild
    Edit(EditArgs),
    /// View calendar events
    View(ViewArgs)
}

#[derive(Debug, Args)]
pub struct AddArgs {
    pub title: String,
    #[clap(num_args=1..=10, allow_hyphen_values=true)]
    pub datetimes: Vec<String>,
}

#[derive(Debug, Args)]
pub struct EditArgs {

}

#[derive(Debug, Args)]
pub struct ViewArgs {

}