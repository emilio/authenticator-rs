#[macro_use]
mod util;

#[cfg(any(target_os = "linux"))]
extern crate libudev;

#[cfg(any(target_os = "linux"))]
#[path = "linux/mod.rs"]
pub mod platform;

#[cfg(any(target_os = "macos"))]
extern crate core_foundation_sys;

#[cfg(any(target_os = "macos"))]
#[path = "macos/mod.rs"]
pub mod platform;

#[cfg(any(target_os = "windows"))]
#[path = "windows/mod.rs"]
pub mod platform;

#[macro_use]
extern crate log;
extern crate rand;
extern crate libc;
extern crate boxfnonce;

mod consts;
mod runloop;
mod u2fprotocol;

mod manager;
pub use manager::U2FManager;

mod capi;
pub use capi::*;
