use std::fs::{self, File};
use std::io::{Error, Read, Write};
use std::{io::ErrorKind, path::PathBuf};

use inquire::Select;
use zip::ZipArchive;

use crate::get_cached_commands_vendordep;
use crate::parser::CliParser;

pub fn unpack_fetched_zip(
    zip_file_path: &PathBuf,
    output_dir: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let project_file = File::open(zip_file_path)
        .expect(format!("Failed to open project file {:?}", zip_file_path).as_str());

    let mut zip_archive = ZipArchive::new(project_file)?;

    std::fs::create_dir_all(output_dir)?;
    println!("Extracting {:?} into {:?}...", zip_file_path, output_dir);
    zip_archive.extract(output_dir.clone()).expect(
        format!(
            "Failed to extract {:?} into {:?}",
            zip_file_path, output_dir
        )
        .as_str(),
    );

    Ok(())
}

pub fn install_project(
    source_dir: &PathBuf,
    parser: &CliParser
) -> Result<(), Box<dyn std::error::Error>> {
    // create output prefix directory in case it doesn't already exist
    std::fs::create_dir_all(&parser.output_prefix()).expect(
        format!(
            "Failed to create output prefix directory {:?}",
            parser.output_prefix()
        )
        .as_str(),
    );

    // prevent creating a robot project in a directory that already exists
    let output_dir = parser.output_prefix()
        .join(parser.name());
    if output_dir.exists() {
        return Err(Error::new(
            ErrorKind::AlreadyExists,
            format!(
                "Project directory {} already exists",
                output_dir.to_str().unwrap()
            ),
        )
        .into());
    }

    let language = parser.language().to_string().to_lowercase();

    let subtype_path_prefix = format!("{}/{}/", source_dir.to_str().unwrap(), language);
    let subtype_paths: Vec<String> = fs::read_dir(&subtype_path_prefix)?
        .into_iter()
        .flatten()
        .map(|x| {
            x.path()
                .to_str()
                .unwrap()
                .to_string()
                .replace(&subtype_path_prefix, "")
        })
        .collect();
    let prompt = format!("Desired {}", parser.project_type().to_string());

    let project_subtype = Select::new(&prompt, subtype_paths).prompt()?;

    let source_dir = format!(
        "{}/{}/{}/",
        source_dir.to_str().unwrap(),
        language,
        project_subtype
    );
    copy_dir::copy_dir(source_dir, &output_dir)?;

    // write team number to wpilib_preferences.json
    let preferences_path = output_dir.join(".wpilib/wpilib_preferences.json");
    let mut preferences_file = std::fs::File::open(&preferences_path)?;
    let mut preferences = String::new();
    preferences_file.read_to_string(&mut preferences)?;
    let team_number = format!("\"teamNumber\": {}", parser.team_number());
    preferences = preferences
        .replace("\"teamNumber\": -1", team_number.as_str());
    let mut preferences_file = std::fs::File::create(preferences_path)?;
    preferences_file.write_all(preferences.as_bytes())?;

    // install WPILibNewCommands.json
    let commands_file = get_cached_commands_vendordep(&parser);
    let vendordep_folder = output_dir.join("vendordeps");
    std::fs::create_dir_all(&vendordep_folder)?;
    std::fs::copy(commands_file, vendordep_folder.join("WPILibNewCommands.json"))?;

    Ok(())
}
