//! OS-specific definitions.

#[cfg(target_os = "linux")] pub use self::linux as target;
#[cfg(target_os = "android")] pub use self::android as target;
#[cfg(target_os = "ios")] pub use self::darwin as target;
#[cfg(target_os = "macos")] pub use self::darwin as target;
#[cfg(target_os = "freebsd")] pub use self::freebsd as target;
#[cfg(target_os = "openbsd")] pub use self::openbsd as target;
#[cfg(target_os = "netbsd")] pub use self::netbsd as target;
#[cfg(target_os = "dragonfly")] pub use self::dragonfly as target;
#[cfg(target_os = "solaris")] pub use self::solaris as target;
#[cfg(target_os = "illumos")] pub use self::illumos as target;

#[cfg(target_os = "linux")] pub mod linux;
#[cfg(target_os = "android")] pub mod android;
#[cfg(target_os = "ios")] pub mod darwin;
#[cfg(target_os = "macos")] pub mod darwin;
#[cfg(target_os = "freebsd")] pub mod freebsd;
#[cfg(target_os = "openbsd")] pub mod openbsd;
#[cfg(target_os = "netbsd")] pub mod netbsd;
#[cfg(target_os = "dragonfly")] pub mod dragonfly;
#[cfg(target_os = "solaris")] pub mod solaris;
#[cfg(target_os = "illumos")] pub mod illumos;
