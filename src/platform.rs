use ash::extensions::{DebugReport, Surface};

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
