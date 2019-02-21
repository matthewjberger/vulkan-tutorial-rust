use ash::{
    extensions::{DebugReport, Surface},
    version::{EntryV1_0, InstanceV1_0},
    vk,
};

use std::os::raw::c_void;

#[cfg(target_os = "windows")]
use ash::extensions::Win32Surface;

#[cfg(all(windows))]
pub fn required_extension_names() -> Vec<*const i8> {
    vec![
        Surface::name().as_ptr(),
        Win32Surface::name().as_ptr(),
        DebugReport::name().as_ptr(),
    ]
}
