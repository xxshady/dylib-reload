use stabby::{libloading::StabbyLibrary, IStable};

pub unsafe fn get_stabbied_fn<R: IStable>(library: &impl StabbyLibrary) -> R {
  let symbol: extern "C" fn() -> R = *library.get_stabbied(&[]).unwrap();
  symbol()
}

pub fn error() {
  let library = (|| -> libloading::Library { todo!() })();

  unsafe {
    get_stabbied_fn(&library);
  }
}
