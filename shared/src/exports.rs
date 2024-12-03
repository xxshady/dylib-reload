use crate::{ModuleId, SliceAllocation, Str};

#[allow(non_camel_case_types, clippy::missing_safety_doc)]
pub trait ___Internal___Exports___ {
  unsafe fn init(host_owner_thread: i64, module: ModuleId);
  unsafe fn exit(allocs: SliceAllocation);
  fn take_cached_allocs_before_exit();
  unsafe fn run_thread_local_dtors();
  fn lock_module_allocator();
  fn spawned_threads_count() -> u64;
}
