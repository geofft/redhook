extern crate libc;

#[cfg(any(target_env = "gnu", target_os = "android"))]
pub mod ld_preload;

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub mod dyld_insert_libraries;
