#[cfg(feature = "build")]
mod build;
#[cfg(feature = "build")]
pub use build::provide;

#[cfg(feature = "normal")]
#[macro_export]
macro_rules! get {
  () => {
    env!("__CRATE_COMPILATION_INFO__")
  };
}
