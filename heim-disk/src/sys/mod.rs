cfg_if::cfg_if! {
    if #[cfg(unix)] {
        mod unix;
        pub use self::unix::*;
    }
}

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;

        pub use self::linux::*;
    } else if #[cfg(target_os = "windows")] {
        mod windows;

        pub use self::windows::*;
    } else if #[cfg(target_os = "macos")] {
        mod macos;

        pub use self::macos::*;
    } else if #[cfg(target_os = "android")] {
        // testing
        mod linux;

        pub use self::linux::*;
    } else {
        compile_error!("Unsupported OS");
    }
}
