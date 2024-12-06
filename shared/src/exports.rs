use crate::{ModuleId, SliceAllocation};

#[allow(non_camel_case_types, clippy::missing_safety_doc)]
pub trait ___Internal___Exports___ {
  unsafe fn init(host_owner_thread: usize, module: ModuleId);
  unsafe fn exit(allocs: SliceAllocation);
  fn take_cached_allocs_before_exit();
  fn lock_module_allocator();

  // currently linux-only
  fn spawned_threads_count() -> u64;
  unsafe fn run_thread_local_dtors();
}
