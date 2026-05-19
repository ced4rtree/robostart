use crate::{cache, config::Config};

mod fetcher;
mod unpack;

pub async fn create_project(config: &Config) -> anyhow::Result<()> {
    // fetch selected project from github into cache
    fetcher::fetch_project(config).await?;

    unpack::unpack_fetched_zip(
        &cache::get_project_zip_path(config)?,
        &cache::get_project_unzipped_path(config)?,
    )?;

    // transfer project from cache into install dir
    unpack::install_project(
        &cache::get_project_unzipped_path(config)?,
        config
    )?;

    Ok(())
}
