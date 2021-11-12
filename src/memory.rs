use std::{ffi::{OsStr, c_void}, mem::size_of, ptr::null};

pub use windows::Win32::System::Memory::{
    self,
    FILE_MAP,
    FILE_MAP_READ,
    FILE_MAP_WRITE,
    PAGE_READWRITE,
    PAGE_PROTECTION_FLAGS,
    MEMORY_BASIC_INFORMATION,
};

use crate::*;

#[allow(non_snake_case)]
pub fn MapViewOfFile(hfilemappingobject: HANDLE, 
    dwdesiredaccess: FILE_MAP, 
    dwfileoffsethigh: u32, 
    dwfileoffsetlow: u32, 
    dwnumberofbytestomap: usize
) -> Result<*mut c_void, Error> {
    
    let v = unsafe {
        Memory::MapViewOfFile(
            hfilemappingobject,
            dwdesiredaccess,
            dwfileoffsethigh,
            dwfileoffsetlow,
            dwnumberofbytestomap
        )
    };

    if v.is_null() {
        return Err(Error::from_win32());
    }

    Ok(v)
}

#[allow(non_snake_case)]
pub fn UnmapViewOfFile(lpbaseaddress: *const c_void) -> Result<(), Error> {
    unsafe {
        Memory::UnmapViewOfFile(lpbaseaddress).ok()
    }
}

#[allow(non_snake_case)]
pub fn CreateFileMappingW<S: AsRef<OsStr>>(hfile: HANDLE, 
    lpfilemappingattributes: Option<&SECURITY_ATTRIBUTES>, 
    flprotect: PAGE_PROTECTION_FLAGS, 
    dwmaximumsizehigh: u32, 
    dwmaximumsizelow: u32, 
    lpname: S
) -> Result<HANDLE, Error> {

    let mut wchar_str = utf8_to_wchar(lpname)?;

    let sec_attr = match lpfilemappingattributes {
        Some(s) => s as *const SECURITY_ATTRIBUTES,
        None => null(),
    };

    let h = unsafe { Memory::CreateFileMappingW(
        hfile,
        sec_attr,
        flprotect,
        dwmaximumsizehigh,
        dwmaximumsizelow,
        PWSTR(wchar_str.as_mut_ptr()),
    )};

    h.ok()
}

#[allow(non_snake_case)]
pub fn VirtualQuery(lpaddress: *const c_void, 
    lpbuffer: &mut MEMORY_BASIC_INFORMATION, 
) -> Result<(), Error> {

    let bytes_written = unsafe {
        Memory::VirtualQuery(
            lpaddress,
            lpbuffer as *mut MEMORY_BASIC_INFORMATION,
            size_of::<MEMORY_BASIC_INFORMATION>(),
        )
    };

    if (bytes_written as usize) < size_of::<MEMORY_BASIC_INFORMATION>() {
        return Err(Error::from_win32());
    }

    Ok(())
}