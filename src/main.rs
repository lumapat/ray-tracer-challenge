use std::{
    thread,
    time,
};
use std::error::Error;
use structopt::StructOpt;

mod basic;

use basic::tuple::{
    BasicTuple,
    Tuple,
};

#[derive(Debug, StructOpt)]
#[structopt(name = "global options", about = "TODO")]
struct GlobalOptions {
    /// Run without committing any changes
    #[structopt(short, long)]
    no_commit: bool,

    /// Set verbose level
    #[structopt(short, long)]
    verbose: bool,
}

#[derive(StructOpt)]
#[structopt(name = "arguments", about = "TODO")]
struct Args {
    #[structopt(flatten)]
    globals: GlobalOptions,

    /// 'Putting It Together' by Chapter
    #[structopt(subcommand)]
    chapter_summary: ChapterSummary
}

#[derive(Debug, StructOpt)]
#[structopt(about = "TODO!")]
enum ChapterSummary {
    #[structopt(name = "ch1")]
    ChapterOne {
        // TODO: Don't think there's anything
        position: BasicTuple,
        velocity: BasicTuple,
    },
}

fn process_command(chapter_summary: &ChapterSummary, _globals: &GlobalOptions) -> Result<(), Box<dyn Error>> {
    match chapter_summary {
        ChapterSummary::ChapterOne{position, velocity} => {
            let sleep_duration = time::Duration::from_secs(1);
            let mut new_position = basic::point::Point::from(position);
            let mut new_velocity = basic::vector::Vector::from(velocity);
            let gravity = basic::vector::vector(0.0, -0.01, 0.0);

            while new_position.tuple().y > 0.0 {
                new_position = new_position + new_velocity.clone();
                new_velocity = new_velocity + gravity.clone();

                println!("Position: {:#?}, velocity: {:#?}", new_position, new_velocity);
                thread::sleep(sleep_duration);
            }
        },
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::SimpleLogger::new().init().unwrap();

    let args = Args::from_args();
    process_command(&args.chapter_summary, &args.globals)?;

    Ok(())
}