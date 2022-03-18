#[doc(hidden)]
pub use windows::core::{*};
#[doc(hidden)]
pub use windows::Win32::Foundation::{
    self, *,
};

mod file_system;
pub use file_system::*;
mod memory;
pub use memory::*;
mod security;
pub use security::*;
mod system_services;
pub use system_services::*;

use core::result::Result;

#[allow(non_snake_case)]
pub fn CloseHandle(h: HANDLE) -> Result<(), Error> {
    unsafe { Foundation::CloseHandle(h).ok() }
}
