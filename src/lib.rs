extern crate libc;

use std::io;
use std::mem;
use std::default::Default;

use libc::{c_int};

pub use os::{cc_t,speed_t,tcflag_t}; // types
pub use os::{VEOF,VEOL,VERASE,VINTR,VKILL,VMIN,VQUIT,VSTART,VSTOP,VSUSP,VTIME}; // c_cc subscripts
pub use os::{BRKINT,ICRNL,IGNBRK,IGNCR,IGNPAR,INLCR,INPCK,ISTRIP,IXANY,IXOFF,IXON,PARMRK}; // input modes
pub use os::{OPOST,ONLCR,OCRNL,ONOCR,ONLRET,OFILL,NLDLY,NL0,NL1,CRDLY,CR0,CR1,CR2,CR3,TABDLY,TAB0,TAB1,TAB2,TAB3,BSDLY,BS0,BS1,VTDLY,VT0,VT1,FFDLY,FF0,FF1}; // output modes
pub use os::{B0,B50,B75,B110,B134,B150,B200,B300,B600,B1200,B1800,B2400,B4800,B9600,B19200,B38400}; // baud rate selection
pub use os::{CSIZE,CS5,CS6,CS7,CS8,CSTOPB,CREAD,PARENB,PARODD,HUPCL,CLOCAL}; // control modes
pub use os::{ECHO,ECHOE,ECHOK,ECHONL,ICANON,IEXTEN,ISIG,NOFLSH,TOSTOP}; // local modes
pub use os::{TCSANOW,TCSADRAIN,TCSAFLUSH}; // attribute selection
pub use os::{TCIFLUSH,TCIOFLUSH,TCOFLUSH,TCIOFF,TCION,TCOOFF,TCOON}; // line control

pub mod ffi;
pub mod os;


pub type Termios = os::termios;

impl Termios {
  pub fn zeroed() -> Self {
    unsafe { mem::zeroed() }
  }

  pub fn from_fd(fd: c_int) -> io::Result<Termios> {
    let mut termios = Termios::zeroed();

    match tcgetattr(fd, &mut termios) {
      Ok(_) => Ok(termios),
      Err(err) => Err(err)
    }
  }
}

impl Default for Termios {
  fn default() -> Self {
    Termios::zeroed()
  }
}


pub fn tcgetattr(fd: c_int, termios: &mut Termios) -> io::Result<()> {
  io_result(unsafe { ffi::tcgetattr(fd, termios) })
}

pub fn tcsetattr(fd: c_int, action: c_int, termios: &Termios) -> io::Result<()> {
  io_result(unsafe { ffi::tcsetattr(fd, action, termios) })
}

pub fn tcsendbreak(fd: c_int, duration: c_int) -> io::Result<()> {
  io_result(unsafe { ffi::tcsendbreak(fd, duration) })
}

pub fn tcdrain(fd: c_int) -> io::Result<()> {
  io_result(unsafe { ffi::tcdrain(fd) })
}

pub fn tcflush(fd: c_int, queue_selector: c_int) -> io::Result<()> {
  io_result(unsafe { ffi::tcflush(fd, queue_selector) })
}

pub fn tcflow(fd: c_int, action: c_int) -> io::Result<()> {
  io_result(unsafe { ffi::tcflow(fd, action) })
}

pub fn cfmakeraw(termios: &mut Termios) {
  unsafe { ffi::cfmakeraw(termios) };
}

pub fn cfgetispeed(termios: &Termios) -> speed_t {
  unsafe { ffi::cfgetispeed(termios) }
}

pub fn cfgetospeed(termios: &Termios) -> speed_t {
  unsafe { ffi::cfgetospeed(termios) }
}

pub fn cfsetispeed(termios: &mut Termios, speed: speed_t) -> io::Result<()> {
  io_result(unsafe { ffi::cfsetispeed(termios, speed) })
}

pub fn cfsetospeed(termios: &mut Termios, speed: speed_t) -> io::Result<()> {
  io_result(unsafe { ffi::cfsetospeed(termios, speed) })
}

pub fn cfsetspeed(termios: &mut Termios, speed: speed_t) -> io::Result<()> {
  io_result(unsafe { ffi::cfsetspeed(termios, speed) })
}


#[inline]
fn io_result(result: c_int) -> io::Result<()> {
  match result {
    0 => Ok(()),
    _ => Err(io::Error::last_os_error())
  }
}
