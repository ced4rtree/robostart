use std::fs::{self, File};
use std::io::{Error, Read, Write};
use std::path::Path;
use std::{io::ErrorKind, path::PathBuf};

use anyhow::Context;
use inquire::Select;
use zip::ZipArchive;

use crate::{get_cached_commands_vendordep, get_cached_gitignore};
use crate::parser::CliParser;

pub fn unpack_fetched_zip(
    zip_file_path: &PathBuf,
    output_dir: &PathBuf,
) -> anyhow::Result<()> {
    let project_file = &File::open(zip_file_path)
        .with_context(|| format!("Failed to open project file {:?}", zip_file_path))?;

    let mut zip_archive = ZipArchive::new(project_file)
        .with_context(|| format!("Failed to create zip archive from {:?}", project_file))?;

    std::fs::create_dir_all(output_dir)
        .with_context(|| format!("Failed to create directory {:?}", output_dir))?;
    println!("Extracting {:?} into {:?}...", zip_file_path, output_dir);
    zip_archive.extract(output_dir.clone())
        .with_context( || format!(
            "Failed to extract {:?} into {:?}",
            zip_file_path, output_dir
        ))?;

    Ok(())
}

pub fn install_project(
    source_dir: &Path,
    parser: &CliParser
) -> anyhow::Result<()> {
    // create output prefix directory in case it doesn't already exist
    std::fs::create_dir_all(parser.output_prefix()).with_context(
        || format!(
            "Failed to create output prefix directory {:?}",
            parser.output_prefix()
        )
    )?;

    // prevent creating a robot project in a directory that already exists
    let output_dir = parser.output_prefix()
        .join(parser.name());
    if output_dir.exists() {
        return Err(Error::new(
            ErrorKind::AlreadyExists,
            format!("Project directory {:?} already exists", output_dir),
        )    .into());
    }

    let language = parser.language().to_string().to_lowercase();

    // Zips are organized into different directories for different languages.
    // "Subtype" here means which specific template/example, e.g. gyro, commandhatchbot, etc.
    let subtype_path_prefix = format!("{:?}/{}/", source_dir, language);
    let subtype_paths: Vec<String> = fs::read_dir(&subtype_path_prefix)
        .with_context(|| format!("While installing unzipped project from /tmp, Failed to read contents of directory: {:?}", subtype_path_prefix))?
        .flatten()
        .map(|x| {
            x.path()
                .to_str()
                .unwrap()
                .to_string()
                .replace(&subtype_path_prefix, "")
        })
        .collect();
    let prompt = format!("Desired {}", parser.project_type());

    let project_subtype = Select::new(&prompt, subtype_paths)
        .prompt()
        .with_context(|| "Failed to retrieve input data for which project subtype to initialize.")?;

    let source_dir = format!(
        "{:?}/{}/{}/",
        source_dir,
        language,
        project_subtype
    );
    copy_dir::copy_dir(&source_dir, &output_dir)
        .with_context(|| format!("Failed to copy {:?} into {:?} while installing project.", source_dir, output_dir))?;

    // write team number to wpilib_preferences.json
    let preferences_path = output_dir.join(".wpilib/wpilib_preferences.json");
    let mut preferences_file = std::fs::File::open(&preferences_path)
        .with_context(|| format!("Failed to open preferences file: {:?}", preferences_path))?;
    let mut preferences = String::new();
    preferences_file.read_to_string(&mut preferences)
        .with_context(|| format!("Failed to read initial contents of the project's preferences file: `{:?}'.", preferences_file))?;
    let team_number = format!("\"teamNumber\": {}", parser.team_number());
    preferences = preferences
        .replace("\"teamNumber\": -1", team_number.as_str());
    let mut preferences_file = std::fs::File::create(preferences_path)
        .with_context(|| format!("Failed to overwrite the project's old preferences file: `{:?}'.", preferences_file))?;
    preferences_file.write_all(preferences.as_bytes())
        .with_context(|| format!("Failed to write updated preferences to the project's preferences file: `{:?}'.", preferences_file))?;

    // install gitignore
    let cached_gitignore = get_cached_gitignore();
    let project_gitignore = output_dir.join(".gitignore");
    std::fs::copy(&cached_gitignore, &project_gitignore)
        .with_context(|| format!("Failed to copy gitignore file {:?} to {:?}", cached_gitignore, project_gitignore))?;

    // install WPILibNewCommands.json
    let commands_file = get_cached_commands_vendordep(parser);
    let vendordep_folder = output_dir.join("vendordeps");
    std::fs::create_dir_all(&vendordep_folder)?;
    std::fs::copy(commands_file, vendordep_folder.join("WPILibNewCommands.json"))?;

    Ok(())
}
