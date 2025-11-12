// Used this video as main reference https://www.youtube.com/watch?v=fD9ptABVQbI
use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct CalArgs {
    #[clap(subcommand)]
    pub event: EventOptions
}

#[derive(Debug, Subcommand)]
pub enum EventOptions {
    /// Adds event to the calendar
    Add(AddArgs)
    // Use wild args to find name using regex
    // https://crates.io/crates/wild
    // Edit {},
    // View {}
}

#[derive(Debug, Args)]
pub struct DateFormat {
    pub start_datetime: String,
    pub end_datetime: Option<String>
}

#[derive(Debug, Args)]
pub struct AddArgs {
    pub title: String,
    #[command(flatten)]
    pub date: DateFormat
}
