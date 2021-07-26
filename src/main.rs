use std::error::Error;
use structopt::StructOpt;

mod basic;

use basic::tuple::BasicTuple;

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
        ChapterSummary::ChapterOne{position, ..} => println!("{:#?}", position),
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::SimpleLogger::new().init().unwrap();

    let args = Args::from_args();
    process_command(&args.chapter_summary, &args.globals)?;

    Ok(())
}