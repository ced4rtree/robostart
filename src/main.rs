use std::{fs::{self, File}, io::{Error, ErrorKind}, path::PathBuf};

use inquire::Select;
use parser::{CliParser, ProjectType};
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

    let random_string: String = rand::random_iter::<u8>()
        .take(8)
        .into_iter()
        .map(|x| x.to_string())
        .collect();
    let tmp_output_dir = PathBuf::from(format!("/tmp/robostart-{random_string}/"));
    fs::create_dir_all(&tmp_output_dir)?;
    println!(
        "Extracting {:?} into {:?}...",
        get_project_file_path(&parser),
        tmp_output_dir
    );
    zip_archive.extract(tmp_output_dir.clone())
        .expect(format!(
            "Failed to extract {:?} into {:?}",
            get_project_file_path(&parser),
            tmp_output_dir
        ).as_str());

    let project_type = match parser.project_type() {
        ProjectType::Example => "examples",
        ProjectType::Template => "templates",
    };
    let subtype_path_prefix = format!("{}{}/", tmp_output_dir.to_str().unwrap(), project_type);
    let subtype_paths: Vec<String> = fs::read_dir(tmp_output_dir.join(project_type))?
        .into_iter()
        .flatten()
        .map(|x| x.path().to_str().unwrap().to_string().replace(&subtype_path_prefix, ""))
        .collect();
    let prompt = format!(
        "Desired {}",
        project_type.to_string()
    );
    let project_subtype = Select::new(&prompt, subtype_paths)
        .prompt()?;

    let source_dir = format!(
        "{}{}/{}/",
        tmp_output_dir.to_str().unwrap(),
        project_type,
        project_subtype
    );
    std::fs::rename(source_dir, output_dir)?;

    println!("Project successfully created!");

    Ok(())
}
