use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("libloading error")]
  Libloading(#[from] libloading::Error),
}
