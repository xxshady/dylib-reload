use stabby::{libloading::StabbyLibrary, IStable};

pub fn unrecoverable(message: &'static str) -> ! {
  eprintln!("something unrecoverable happened: {message}");
  eprintln!("aborting");
  std::process::abort();
}

pub fn cstr_bytes(str: &str) -> Vec<u8> {
  [str.as_bytes(), &[0]].concat()
}

pub unsafe fn get_stabbied_fn<F: IStable + Copy>(library: &impl StabbyLibrary, name: &str) -> F {
  let symbol = library.get_stabbied(&cstr_bytes(name)).unwrap_or_else(|e| {
    panic!("Failed to get {name} symbol from library");
  });
  *symbol
}
