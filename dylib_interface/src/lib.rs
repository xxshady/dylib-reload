#[cfg(feature = "build")]
pub mod host;
#[cfg(feature = "build")]
pub mod module;
#[cfg(feature = "build")]
mod shared;

#[cfg(feature = "normal")]
#[macro_export]
macro_rules! include_generated {
  ($mod_name:ident, $file_name:literal) => {
    mod $mod_name {
      include!(concat!(env!("OUT_DIR"), $file_name));
    }
  };
}
