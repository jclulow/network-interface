#[cfg(windows)]
pub mod hex;
#[cfg(any(target_os = "linux", target_os = "illumos", target_os = "macos"))]
mod unix;

#[cfg(any(target_os = "linux", target_os = "illumos", target_os = "macos"))]
pub use unix::*;
