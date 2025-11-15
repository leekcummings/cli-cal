// Used this video as main reference https://www.youtube.com/watch?v=fD9ptABVQbI
use clap:: {
    Args,
    Parser,
    Subcommand
};

/// The datetime format used in all calendar events
// #[derive(Debug, Args)]
// pub struct DateFormat {
//     pub start_datetime: String,
//     pub end_datetime: Option<String>
// }

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
    // Removed temp because I might need a different approach
    // #[command(flatten)]
    // pub date: DateFormat
    #[clap(num_args=1..=10, allow_hyphen_values=true)]
    pub datetimes: Vec<String>,
}

#[derive(Debug, Args)]
pub struct EditArgs {

}

#[derive(Debug, Args)]
pub struct ViewArgs {

}