use crate::utils::Result;
use clap::ArgMatches;
use dialoguer::Confirm;
use lib_fpm::{config::Config, database};

pub fn reset(sub_matches: &ArgMatches, config: &Config) -> Result<()> {
    let force = sub_matches.get_flag("force");

    if force
        || Confirm::new()
            .with_prompt("Are you sure you want to reset the entire database? This is irreversible")
            .interact()?
    {
        database::reset_database(config)?;
    }

    Ok(())
}
