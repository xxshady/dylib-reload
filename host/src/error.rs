use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("libloading error")]
  Libloading(#[from] libloading::Error),

  #[error(
    "module is compiled with different rustc version:\n\
    {0}\n\
    expected:\n\
    {1}"
  )]
  ModuleCompilationMismatch(String, String),
}
