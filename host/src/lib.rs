use stabby::{libloading::StabbyLibrary, IStable};

pub unsafe fn get_stabbied_fn(library: &impl StabbyLibrary, name: &str) -> impl IStable + Copy {
  let symbol = library.get_stabbied(&[]).unwrap();
  *symbol
}
