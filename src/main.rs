use std::error::Error;
use structopt::StructOpt;

mod basic;

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
    }
}

fn process_command(chapter_summary: &ChapterSummary, globals: &GlobalOptions) -> Result<(), Box<dyn Error>> {
    match chapter_summary {
        ChapterSummary::ChapterOne{} => println!("Good job!!"),
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::SimpleLogger::new().init().unwrap();

    let args = Args::from_args();
    process_command(&args.chapter_summary, &args.globals)?;

    Ok(())
}