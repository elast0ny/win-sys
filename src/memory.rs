use std::{
    ffi::{c_void, OsStr},
    mem::size_of,
    ptr::null,
};

#[doc(hidden)]
pub use windows::Win32::System::Memory::{
    self, FILE_MAP, FILE_MAP_ALL_ACCESS, FILE_MAP_READ, FILE_MAP_WRITE, MEMORY_BASIC_INFORMATION,
    PAGE_PROTECTION_FLAGS, PAGE_READWRITE,
};

use crate::*;

use core::result::Result;

pub struct ViewOfFile(*mut c_void);
impl ViewOfFile {
    pub fn as_mut_ptr(&self) -> *mut c_void {
        self.0
    }
}
impl core::fmt::Pointer for ViewOfFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        core::fmt::Pointer::fmt(&self.0, f)
    }
}
impl Drop for ViewOfFile {
    fn drop(&mut self) {
        unsafe {
            Memory::UnmapViewOfFile(self.0);
        }
    }
}

#[allow(non_snake_case)]
pub fn MapViewOfFile(
    hfilemappingobject: HANDLE,
    dwdesiredaccess: FILE_MAP,
    dwfileoffsethigh: u32,
    dwfileoffsetlow: u32,
    dwnumberofbytestomap: usize,
) -> Result<ViewOfFile, Error> {
    let v = unsafe {
        Memory::MapViewOfFile(
            hfilemappingobject,
            dwdesiredaccess,
            dwfileoffsethigh,
            dwfileoffsetlow,
            dwnumberofbytestomap,
        )
    };

    if v.is_null() {
        return Err(Error::from_win32());
    }

    Ok(ViewOfFile(v))
}

pub struct FileMapping(HANDLE);
impl FileMapping {
    pub fn as_handle(&self) -> HANDLE {
        self.0
    }
}
impl core::fmt::UpperHex for FileMapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        core::fmt::UpperHex::fmt(&self.0 .0, f)
    }
}
impl Drop for FileMapping {
    fn drop(&mut self) {
        let _ = CloseHandle(self.0);
    }
}

#[allow(non_snake_case)]
pub fn CreateFileMapping<S: AsRef<OsStr>>(
    hfile: HANDLE,
    lpfilemappingattributes: Option<&SECURITY_ATTRIBUTES>,
    flprotect: PAGE_PROTECTION_FLAGS,
    dwmaximumsizehigh: u32,
    dwmaximumsizelow: u32,
    lpname: S,
) -> Result<FileMapping, Error> {
    let sec_attr = match lpfilemappingattributes {
        Some(s) => s as *const SECURITY_ATTRIBUTES,
        None => null(),
    };

    let h = unsafe {
        Memory::CreateFileMappingW(
            hfile,
            sec_attr,
            flprotect,
            dwmaximumsizehigh,
            dwmaximumsizelow,
            lpname.as_ref(),
        )
    };

    Ok(FileMapping(h.ok()?))
}

#[allow(non_snake_case)]
pub fn OpenFileMapping<S: AsRef<OsStr>>(
    desired_access: FILE_MAP,
    inherit_handle: bool,
    name: S,
) -> Result<FileMapping, Error> {
    let h = unsafe {
        Memory::OpenFileMappingW(
            desired_access.0,
            inherit_handle,
            name.as_ref(),
        )
    };

    Ok(FileMapping(h.ok()?))
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[allow(non_snake_case)]
pub fn VirtualQuery(
    lpaddress: *const c_void,
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
