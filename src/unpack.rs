use std::io::Error;
use std::{io::ErrorKind, path::PathBuf};
use std::fs::{self, File};

use inquire::Select;
use zip::ZipArchive;

use crate::parser::ProjectType;

pub fn unpack_zip(zip_file_path: &PathBuf, output_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let project_file = File::open(zip_file_path)
        .expect(format!(
            "Failed to open project file {:?}",
            zip_file_path
        ).as_str());

    let mut zip_archive = ZipArchive::new(project_file)?;

    println!(
        "Extracting {:?} into {:?}...",
        zip_file_path,
        output_dir
    );
    zip_archive.extract(output_dir.clone())
        .expect(format!(
            "Failed to extract {:?} into {:?}",
            zip_file_path,
            output_dir
        ).as_str());
    
    Ok(())
}

pub fn install_project(
    source_dir: &PathBuf,
    output_prefix: &PathBuf,
    project_name: &String,
    project_type: &ProjectType
) -> Result<(), Box<dyn std::error::Error>> {
    // create output prefix directory
    std::fs::create_dir_all(&output_prefix).expect(
        format!(
            "Failed to create output prefix directory {:?}",
            output_prefix
        ).as_str()
    );

    // prevent creating a robot project in a directory that already exists
    let output_dir = output_prefix.join(project_name);
    if output_dir.exists() {
        return Err(Error::new(ErrorKind::AlreadyExists, format!(
            "Project directory {} already exists",
            output_dir.to_str().unwrap()
        )).into());
    }

    let project_type = match project_type {
        ProjectType::Example => "examples",
        ProjectType::Template => "templates",
    };
    let subtype_path_prefix = format!("{}{}/", source_dir.to_str().unwrap(), project_type);
    let subtype_paths: Vec<String> = fs::read_dir(source_dir.join(project_type))?
        .into_iter()
        .flatten()
        .map(|x| x.path().to_str().unwrap().to_string().replace(&subtype_path_prefix, ""))
        .filter(|x| !x.ends_with(".json"))
        .collect();
    let prompt = format!(
        "Desired {}",
        project_type.to_string()
    );
    let project_subtype = Select::new(&prompt, subtype_paths)
        .prompt()?;

    let source_dir = format!(
        "{}{}/{}/",
        source_dir.to_str().unwrap(),
        project_type,
        project_subtype
    );
    std::fs::rename(source_dir, output_dir)?;
    
    Ok(())
}
