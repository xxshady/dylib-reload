use std::{
  fmt::{Debug, Formatter, Result as FmtResult},
  marker::PhantomData,
  mem::ManuallyDrop,
  ops::Deref,
};

use libloading::Library;

pub trait ModuleExportsForHost {
  fn new(library: &Library) -> Self;
}

/// Smart pointer to a value owned by module.
/// Think of it as a reference with lifetime of the module.
///
/// # Examples
/// ```
/// // a slice of memory owned by module
/// #[repr(C)]
/// struct SomeMemory {
///   ptr: *const u8,
///   len: usize,
/// }
///
/// let slice: ModuleValue<'_, SomeMemory> = module.call_main().unwrap();
///
/// // .unload() frees memory of the module
/// module.unload().unwrap();
///
/// // compile error, this memory slice is deallocated by .unload
/// dbg!(slice);
/// ```
pub struct ModuleValue<'module, T> {
  module: PhantomData<&'module ()>,

  /// `ManuallyDrop` to prevent double free of module memory.
  /// Module will deallocate anything it has allocated when it's unloaded, and it
  /// doesn't currently know if anything has been moved from it.
  value: ManuallyDrop<T>,
}

impl<'module, T> ModuleValue<'module, T> {
  pub fn new<'lt>(value: T) -> ModuleValue<'lt, T>
  where
    'module: 'lt,
  {
    Self {
      module: PhantomData,
      value: ManuallyDrop::new(value),
    }
  }
}

impl<T> Debug for ModuleValue<'_, T>
where
  T: Debug,
{
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{:?}", *self.value)
  }
}

impl<T> Deref for ModuleValue<'_, T> {
  type Target = T;

  fn deref(&self) -> &T {
    &self.value
  }
}
