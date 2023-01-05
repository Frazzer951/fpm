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
