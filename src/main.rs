use std::{fs::File, io::{Error, ErrorKind}, path::PathBuf};

use parser::CliParser;
use zip::read::ZipArchive;

mod fetcher;
mod parser;

pub fn get_project_cache() -> PathBuf {
    PathBuf::from(format!(
        "{}/.cache/robostart/",
        std::env::home_dir().unwrap().display()
    ).as_str())
}

pub fn get_project_file_name(parser: &CliParser) -> PathBuf {
    PathBuf::from(format!(
        "{:?}-{:?}-{}.zip",
        parser.language(),
        parser.project_type(),
        parser.wpilib_version(),
    ).as_str())
}

pub fn get_project_file_path(parser: &CliParser) -> PathBuf {
    get_project_cache().join(get_project_file_name(&parser))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = CliParser::new()?;

    // fetch selected project from artifactory
    fetcher::fetch_project(&parser).await?;

    // populate the desired directory with selected project
    std::fs::create_dir_all(parser.output_prefix()).expect(
        format!(
            "Failed to create output prefix directory {}",
            parser.output_prefix().display()
        ).as_str(),
    );

    let project_file = File::open(get_project_file_path(&parser))
        .expect(format!(
            "Failed to open project file {}",
            get_project_file_path(&parser).display()
        ).as_str());

    let mut zip_archive = ZipArchive::new(project_file)?;

    let output_dir = parser.output_prefix().join(parser.name());
    if output_dir.exists() {
        return Err(Error::new(ErrorKind::AlreadyExists, format!(
            "Project directory {} already exists",
            output_dir.to_str().unwrap()
        )).into());
    }

    println!(
        "Extracting {:?} into {:?}...",
        get_project_file_path(&parser),
        output_dir
    );
    zip_archive.extract(output_dir.clone())
        .expect(format!(
            "Failed to extract {:?} into {:?}",
            get_project_file_path(&parser),
            output_dir
        ).as_str());

    println!("Project successfully created!");

    Ok(())
}
