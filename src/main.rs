use std::path::PathBuf;

use parser::CliParser;

mod fetcher;
mod parser;
mod unpack;

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

    let random_string: String = rand::random_iter::<u8>()
        .take(8)
        .into_iter()
        .map(|x| x.to_string())
        .collect();
    let tmp_output_dir = PathBuf::from(format!("/tmp/robostart-{random_string}/"));
    unpack::unpack_zip(
        &get_project_file_path(&parser),
        &tmp_output_dir
    )?;
    unpack::install_project(
        &tmp_output_dir,
        &parser.output_prefix(),
        &parser.name(),
        &parser.project_type()
    )?;

    println!("Project successfully created!");

    Ok(())
}
