// Used this video as main reference https://www.youtube.com/watch?v=fD9ptABVQbI
use clap:: {
    Args,
    Parser,
    Subcommand
};

// Not using this I think unless I can get multiple clap arguments into here at once
// fn date_parse(raw_date: &str) -> Result<chrono::DateTime<Utc>, anyhow::Error> {
//     println!("{}", &raw_date);
//     parse_with(raw_date, &Utc, DEFAULT_TIME)
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
    #[clap(num_args=1..=10)]
    pub datetimes: Vec<String>,
    #[clap(short)]
    pub location: Option<String>,
    /// Make event daily, weekly, or monthly
    #[clap(short)]
    pub repeat: Option<String>,
    #[clap(short, num_args=1..)]
    pub description: Option<String>
}

#[derive(Debug, Args)]
pub struct EditArgs {

}

#[derive(Debug, Args)]
pub struct ViewArgs {

}