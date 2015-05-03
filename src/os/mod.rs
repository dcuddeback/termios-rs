//! OS-specific definitions.

#[cfg(target_os = "linux")] pub use self::linux as target;
#[cfg(target_os = "macos")] pub use self::macos as target;

#[cfg(target_os = "linux")] pub mod linux;
#[cfg(target_os = "macos")] pub mod macos;
