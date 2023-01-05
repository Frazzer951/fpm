use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Indicatif(#[from] indicatif::style::TemplateError),

    #[error(transparent)]
    Fpm(#[from] lib_fpm::error::Error),
}

pub fn create_spinner(msg: &str) -> Result<ProgressBar> {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(80));
    pb.set_style(
        ProgressStyle::with_template("[{elapsed}] {spinner:.blue} - {msg}")?
            .tick_strings(&["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"]),
    );

    pb.set_message(msg.to_owned());
    Ok(pb)
}
