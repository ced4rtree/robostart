use clap::Parser;
use fetcher::{Language, ProjectType};

mod fetcher;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Language to initialize
    #[arg(short, long)]
    language: Language,

    /// Type of project to initalize
    #[arg(short = 't', long = "type")]
    project_type: ProjectType,

    /// What version to download
    #[arg(short, long)]
    wpilib_version: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    fetcher::fetch_project(&args).await?;

    Ok(())
}
