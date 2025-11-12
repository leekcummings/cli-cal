mod args;

use clap::Parser;
use crate::args::{CalArgs, EventOptions};

fn main() {
    let cli = CalArgs::parse();

    match &cli.event {
        EventOptions::Add(args) => {
            println!("{:?}", args);
        }
    }

    // Failed code graveyard RIP
    // let args = CalArgs::parse();

    // match &args.event {
    //     EventOptions::Add { title, start, end } => {
    //         println!("{title} {start} {:?}", end);
    //     }
    //     _ => {}
    // }

    // let event = Event {
    //     title: args,
    //     start_date: args.start
    // }
}
