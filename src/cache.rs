use std::path::PathBuf;

use crate::config::Config;

pub fn get_robostart_cache() -> PathBuf {
    PathBuf::from(format!(
        "{}/.cache/robostart/",
        std::env::home_dir().unwrap().display()
    ).as_str())
}

fn get_project_name(config: &Config) -> anyhow::Result<String> {
    let wpilib_version = &config.wpilib_version()?;
    let project_type = &config.project_type()?;
    Ok(format!(
        "{}-{:?}",
        wpilib_version,
        project_type,
    ))
}

// where the cached .zip with all of the examples/templates is located
pub fn get_project_zip_path(config: &Config) -> anyhow::Result<PathBuf> {
    Ok(get_robostart_cache().join(get_project_name(config)? + ".zip"))
}

// where the cached directory with all of the examples/templates is located
pub fn get_project_unzipped_path(config: &Config) -> anyhow::Result<PathBuf> {
    Ok(get_robostart_cache().join(get_project_name(config)?))
}

pub fn get_cached_commands_vendordep(config: &Config) -> anyhow::Result<PathBuf> {
    Ok(get_robostart_cache().join(format!("vendordeps/newcommands-{}.json", config.wpilib_version()?)))
}

pub fn get_cached_gitignore() -> PathBuf {
    get_robostart_cache().join("gitignore")
}
