use stabby::{libloading::StabbyLibrary, IStable};

pub unsafe fn get_stabbied_fn<F: IStable + Copy>(library: &impl StabbyLibrary, name: &str) -> F {
  let symbol = library.get_stabbied(&[]).unwrap();
  *symbol
}
