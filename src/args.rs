// Used this video as main reference https://www.youtube.com/watch?v=fD9ptABVQbI
use clap:: {
    Args,
    ArgAction,
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct CalArgs {
    #[clap(subcommand)]
    pub event: Option<EventOptions>,
    #[clap(short, action=ArgAction::SetTrue, help="Show all available events")]
    pub all: Option<bool>,
    #[clap(short, action=ArgAction::SetTrue, help="Show event locations")]
    pub location: Option<bool>,
    #[clap(short, action=ArgAction::SetTrue, help="Show event repeats")]
    pub repeat: Option<bool>
}

#[derive(Debug, Subcommand)]
pub enum EventOptions {
    /// Add event to your calendar
    Add(AddArgs),
    /// Edit an existing event in calendar (currently not working)
    // Use wild args to find name using regex
    // https://crates.io/crates/wild
    Edit(EditArgs),
    /// View calendar events
    View(ViewArgs)
}

#[derive(Debug, Args)]
pub struct AddArgs {
    // The title of the event
    pub title: String,
    #[clap(num_args=1..=10, help="The start and end time, separated by a comma")]
    pub datetimes: Vec<String>,
    #[clap(short, help="The location of your event")]
    pub location: Option<String>,
    #[clap(short, help="Make event daily, weekly, or monthly (currently not working)")]
    pub repeat: Option<String>,
    #[clap(short, num_args=1.., help="Add an option description for the event")]
    pub description: Option<String>
}

#[derive(Debug, Args)]
pub struct EditArgs {
    // Nothing to show yet...
}

#[derive(Debug, Args)]
pub struct ViewArgs {
    #[clap(short, action=ArgAction::SetTrue, help="Show all available events")]
    pub all: Option<bool>,
    #[clap(short, action=ArgAction::SetTrue, help="Show event locations")]
    pub location: Option<bool>,
    #[clap(short, action=ArgAction::SetTrue, help="Show event repeats")]
    pub repeat: Option<bool>
}