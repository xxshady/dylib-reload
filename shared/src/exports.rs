use crate::{ModuleId, SliceAllocation};

#[allow(non_camel_case_types)]
pub trait ___Internal___Exports___ {
  unsafe fn init(host_owner_thread: i64, module: ModuleId);
  unsafe fn exit(allocs: SliceAllocation);
  fn unloaded() -> bool;
  fn request_cached_allocs();
  unsafe fn run_thread_local_dtors();
}
