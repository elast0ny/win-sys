

use std::ffi::OsStr;

use windows::runtime::{HRESULT, HSTRING};
pub use windows::runtime::{Error, Handle};
pub use windows::Win32::Foundation::{
    self,
    HANDLE,
    PWSTR,
    ERROR_ALREADY_EXISTS,
    ERROR_NO_UNICODE_TRANSLATION,
};

mod file_system;
pub use file_system::*;
mod memory;
pub use memory::*;
mod security;
pub use security::*;
mod system_services;
pub use system_services::*;

pub fn utf8_to_wchar<S: AsRef<OsStr>>(s: S) -> Result<widestring::U16CString, Error> {
    match widestring::U16CString::from_os_str(s) {
        Ok(w) => Ok(w),
        Err(_e) => Err(Error::new(HRESULT(ERROR_NO_UNICODE_TRANSLATION.0),  HSTRING::from("Failed to convert utf8 to widechar"))),
    }
}

#[allow(non_snake_case)]
pub fn CloseHandle(h: HANDLE) -> Result<(), Error> {
    unsafe {
        Foundation::CloseHandle(h).ok()
    }
}