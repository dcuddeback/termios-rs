//! Unsafe FFI bindings.

use libc::{c_int,pid_t};

#[link(name = "c")]
extern "C" {
    pub fn tcgetattr(fd: c_int, termios_p: *mut ::os::target::termios) -> c_int;
    pub fn tcsetattr(fd: c_int, optional_actions: c_int, termios_p: *const ::os::target::termios) -> c_int;
    pub fn tcsendbreak(fd: c_int, duration: c_int) -> c_int;
    pub fn tcdrain(fd: c_int) -> c_int;
    pub fn tcflush(fd: c_int, queue_selector: c_int) -> c_int;
    pub fn tcflow(fd: c_int, action: c_int) -> c_int;
    #[cfg(not(target_os = "solaris"))]
    pub fn cfmakeraw(termios_p: *mut ::os::target::termios);
    pub fn cfgetispeed(termios_p: *const ::os::target::termios) -> ::os::target::speed_t;
    pub fn cfgetospeed(termios_p: *const ::os::target::termios) -> ::os::target::speed_t;
    pub fn cfsetispeed(termios_p: *mut ::os::target::termios, speed: ::os::target::speed_t) -> c_int;
    pub fn cfsetospeed(termios_p: *mut ::os::target::termios, speed: ::os::target::speed_t) -> c_int;
    #[cfg(not(target_os = "solaris"))]
    pub fn cfsetspeed(termios_p: *mut ::os::target::termios, speed: ::os::target::speed_t) -> c_int;
    pub fn tcgetsid(fd: c_int) -> pid_t;
}

#[cfg(target_os = "solaris")]
pub unsafe fn cfmakeraw(termios: *mut ::os::target::termios) {
    use ::os::target::{IMAXBEL, IGNBRK, BRKINT, PARMRK, ISTRIP, INLCR, IGNCR, ICRNL, IXON, OPOST,
        ECHO, ECHONL, ICANON, ISIG, IEXTEN, CSIZE, PARENB, CS8};
    let mut t = *termios as ::os::target::termios;
    t.c_iflag &= !(IMAXBEL|IGNBRK|BRKINT|PARMRK|ISTRIP|INLCR|IGNCR|ICRNL|IXON);
    t.c_oflag &= !OPOST;
    t.c_lflag &= !(ECHO|ECHONL|ICANON|ISIG|IEXTEN);
    t.c_cflag &= !(CSIZE|PARENB);
    t.c_cflag |= CS8;
}

#[cfg(target_os = "solaris")]
pub unsafe fn cfsetspeed(termios_p: *mut ::os::target::termios, speed: ::os::target::speed_t) -> c_int {
    let ret = cfsetispeed(termios_p, speed);
    if ret != 0 {
        return ret;
    }
    cfsetospeed(termios_p, speed)
}
