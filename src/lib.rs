#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

extern "C" {
  // TODO: This isnt present in linux libc
  fn backtrace_symbols_fd(buffer: *const *mut libc::c_void, size: libc::c_int, fd: libc::c_int);
}

/// # Safety
/// This is unsafe, it calls libc functions
#[cfg(all(unix, target_env = "musl"))]
unsafe fn display_trace() {
  libc::exit(libc::EXIT_FAILURE);
}

/// # Safety
/// This is unsafe, it calls libc functions
#[cfg(all(unix, not(target_env = "musl")))]
unsafe fn display_trace() {
  // Borrowed from https://github.com/rust-lang/rust/blob/90677edcba063ee83316ef109e0fe54116015575/compiler/rustc_driver_impl/src/lib.rs#L1297-L1308
  const MAX_FRAMES: usize = 256;
  static mut STACK_TRACE: [*mut libc::c_void; MAX_FRAMES] = [std::ptr::null_mut(); MAX_FRAMES];
  let depth = libc::backtrace(STACK_TRACE.as_mut_ptr(), MAX_FRAMES as i32);
  backtrace_symbols_fd(STACK_TRACE.as_ptr(), depth, 2);
  libc::exit(libc::EXIT_FAILURE);
}

/// # Safety
/// This is unsafe, it calls libc functions
#[napi]
pub unsafe fn init_fault_handler() {
  libc::signal(libc::SIGSEGV, display_trace as usize);
  libc::signal(libc::SIGTRAP, display_trace as usize);
}

/// # Safety
/// This is unsafe, it calls libc functions
#[napi]
pub unsafe fn trigger_signal() {
  std::ptr::null_mut::<i32>().write(42);
}
