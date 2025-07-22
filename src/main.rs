use std::path::PathBuf;

use parser::CliParser;

mod fetcher;
mod parser;
mod unpack;

pub fn get_robostart_cache() -> PathBuf {
    PathBuf::from(format!(
        "{}/.cache/robostart/",
        std::env::home_dir().unwrap().display()
    ).as_str())
}

pub fn get_project_name(parser: &CliParser) -> String {
    format!(
        "{}-{:?}",
        parser.wpilib_version(),
        parser.project_type(),
    )
}

// where the cached .zip with all of the examples/templates is located
pub fn get_project_zip_path(parser: &CliParser) -> PathBuf {
    get_robostart_cache().join(get_project_name(&parser) + ".zip")
}

// where the cached directory with all of the examples/templates is located
pub fn get_project_unzipped_path(parser: &CliParser) -> PathBuf {
    get_robostart_cache().join(get_project_name(&parser))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = CliParser::new()?;

    // fetch selected project from github into cache
    fetcher::fetch_project(&parser).await?;

    unpack::unpack_fetched_zip(
        &get_project_zip_path(&parser),
        &get_project_unzipped_path(&parser),
    )?;

    // transfer project from cache into install dir
    unpack::install_project(
        &get_project_unzipped_path(&parser),
        &parser.output_prefix(),
        &parser.name(),
        &parser.project_type(),
        &parser.language()
    )?;

    println!("Project successfully created!");

    Ok(())
}
