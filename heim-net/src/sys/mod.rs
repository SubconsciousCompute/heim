#[cfg(unix)]
mod unix;

#[cfg(unix)]
pub use self::unix::*;

cfg_if::cfg_if! {
    if #[cfg(any(target_os = "linux", target_os = "android"))] {
        mod linux;

        pub use self::linux::*;
    } else if #[cfg(target_os = "macos")] {
        mod macos;

        pub use self::macos::*;
    } else if #[cfg(target_os = "windows")] {
        mod windows;

        pub use self::windows::*;
    }
}
