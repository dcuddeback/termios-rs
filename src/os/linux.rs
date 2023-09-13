#![allow(non_camel_case_types)]

use libc::{c_uint,c_uchar};

pub type cc_t = c_uchar;
pub type speed_t = c_uint;
pub type tcflag_t = c_uint;

pub use libc::{VEOF,VEOL,VERASE,VINTR,VKILL,VMIN,VQUIT,VSTART,VSTOP,VSUSP,VTIME}; // c_cc subscripts
pub use libc::{BRKINT,ICRNL,IGNBRK,IGNCR,IGNPAR,INLCR,INPCK,ISTRIP,IXANY,IXOFF,IXON,PARMRK}; // input modes
pub use libc::{OPOST,ONLCR,OCRNL,ONOCR,ONLRET}; // output modes
pub use libc::{B0,B50,B75,B110,B134,B150,B200,B300,B600,B1200,B1800,B2400,B4800,B9600,B19200,B38400}; // baud rate selection
pub use libc::{CSIZE,CS5,CS6,CS7,CS8,CSTOPB,CREAD,PARENB,PARODD,HUPCL,CLOCAL}; // control modes
pub use libc::{ECHO,ECHOE,ECHOK,ECHONL,ICANON,IEXTEN,ISIG,NOFLSH,TOSTOP}; // local modes
pub use libc::{TCSANOW,TCSADRAIN,TCSAFLUSH}; // attribute selection
pub use libc::{TCIFLUSH,TCIOFLUSH,TCOFLUSH,TCIOFF,TCION,TCOOFF,TCOON}; // line control
pub use libc::NCCS;

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
#[repr(C)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    c_line: cc_t,
    pub c_cc: [cc_t; NCCS],
    c_ispeed: speed_t,
    c_ospeed: speed_t
}
