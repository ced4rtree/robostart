/// Parse user supplied arguments
use std::{cell::OnceCell, fmt::{Display, Formatter}, path::PathBuf};

use anyhow::Context;

use inquire::{CustomType, Select, Text};
use robostart::AllVariants;

#[derive(robostart::LazyStruct, Debug)]
pub struct Config {
    /// Language to initialize
    #[absent_handler(|_| Select::new(
        "Language: ",
        Language::all_variants().to_vec()
    )
        .prompt()
        .with_context(|| "Failed to prompt for language selection."))]
    language: OnceCell<Language>,

    /// Type of project to initalize
    #[absent_handler(|_| Select::new(
        "Project Type: ",
        ProjectType::all_variants().to_vec()
    )
        .prompt()
        .with_context(|| "Failed to prompt for the project type."))]
    project_type: OnceCell<ProjectType>,

    /// What version to download
    #[absent_handler(|config: &Self| -> Result<String, anyhow::Error> {
        let proj_type = config.project_type()?;
        Text::new(format!("{} Version: ", proj_type).as_str())
            .with_help_message("This will match the corresponding WPILib version, e.g. 2025.3.2")
            .prompt()
            .with_context(|| "Failed to prompt for WPILib version.")
    })]
    wpilib_version: OnceCell<String>,

    /// The parent directory for the new project
    // idk why the compiler can't figure out that this returns an anyhow::Error
    #[absent_handler(|_| -> Result<_, anyhow::Error> {
        let prompt = Text::new("What directory should the project live under?")
            .with_help_message("This is just the parent directory of your project, don't include the project name.")
            .prompt()
            .with_context(|| "Failed to prompt for output prefix.")?;

        // grrr temporary value dropped while borrowed
        let home_dir = std::env::home_dir()
            .with_context(|| "Failed to read user\'s home directory into a `PathBuf`.")?;
        let home_dir = home_dir
            .to_str()
            .with_context(|| "Failed to read home directory path into a string.")?;

        let path_str = prompt.replace("~", home_dir); 

        Ok(PathBuf::from(path_str))
    })]
    output_prefix: OnceCell<PathBuf>,

    /// Name of the new project
    #[absent_handler(|_| Text::new("Project Name: ")
        .prompt()
        .with_context(|| "Failed to prompt for project name."))]
    name: OnceCell<String>,

    /// Your team number
    #[absent_handler(|_| CustomType::new("Team Number: ")
        .with_formatter(&|i: u32| format!("{i}"))
        .with_error_message("Please type a valid integer greater than 0")
        .prompt()
        .with_context(|| "Could not read team number input."))]
    team_number: OnceCell<u32>,
}

/// Which language to use
#[derive(Clone, Debug, AllVariants)]
pub enum Language {
    Java,
    Cpp,
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{self:?}")
    }    
}

/// Which type of project to initialize
#[derive(Clone, Debug, AllVariants)]
pub enum ProjectType {
    Example,
    Template,
}

impl Display for ProjectType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{self:?}")
    }    
}
