use std::{io::Error, fs::File, io::ErrorKind, path::PathBuf};

use clap::Parser;
use fetcher::{Language, ProjectType};
use zip::read::ZipArchive;

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

    /// Where to put the new project
    #[arg(short, long)]
    output_prefix: String,

    /// Name of the new project
    #[arg(short, long)]
    name: String,
}

pub fn get_project_cache() -> PathBuf {
    PathBuf::from(format!(
        "{}/.cache/robostart/",
        std::env::home_dir().unwrap().display()
    ).as_str())
}

pub fn get_project_file_name(args: &Args) -> PathBuf {
    PathBuf::from(format!(
        "{:?}-{:?}-{}.zip",
        args.language,
        args.project_type,
        args.wpilib_version
    ).as_str())
}

pub fn get_project_file_path(args: &Args) -> PathBuf {
    get_project_cache().join(get_project_file_name(&args))
}

fn get_project_output_dir(args: &Args) -> PathBuf {
    let output_prefix = if args.output_prefix.ends_with('/') {
        args.output_prefix.clone()
    } else {
        let mut ret = args.output_prefix.clone();
        ret.push('/');
        ret
    };
    
    PathBuf::from(format!(
        "{}{}",
        output_prefix,
        args.name
    ))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // fetch selected project from artifactory
    fetcher::fetch_project(&args).await?;

    // populate the desired directory with selected project
    std::fs::create_dir_all(args.output_prefix.as_str()).expect(
        format!(
            "Failed to create output prefix directory {}",
            args.output_prefix
        ).as_str(),
    );

    let project_file = File::open(get_project_file_path(&args))
        .expect(format!(
            "Failed to open project file {}",
            get_project_file_path(&args).display()
        ).as_str());

    let mut zip_archive = ZipArchive::new(project_file)?;

    let output_dir = get_project_output_dir(&args);
    if output_dir.exists() {
        return Err(Error::new(ErrorKind::AlreadyExists, format!(
            "Project directory {} already exists",
            output_dir.to_str().unwrap()
        )).into());
    }

    zip_archive.extract(output_dir.clone())
        .expect(format!(
            "Failed to extract {:?} into {:?}",
            get_project_file_path(&args),
            output_dir
        ).as_str());

    Ok(())
}
