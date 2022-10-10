#[cfg_attr(
    all(target_os = "linux", target_pointer_width = "64"),
    path = "linux/linux.rs"
)]
#[cfg_attr(target_os = "windows", path = "windows/windows.rs")]
#[cfg_attr(target_os = "macos", path = "macos/macos.rs")]
mod os_impl;
pub(crate) use os_impl::OsDispatcher;
pub(crate) use os_impl::OsTyper;
