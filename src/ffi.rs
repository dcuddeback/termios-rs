extern crate libc;
use self::libc::{c_void,c_int};

pub use self::os::*;

#[link(name = "c")]
extern "C" {
  pub fn tcgetattr(fd: c_int, termios_p: *mut Termios) -> c_int;
  pub fn tcsetattr(fd: c_int, optional_actions: c_int, termios_p: *const Termios) -> c_int;
  pub fn tcsendbreak(fd: c_int, duration: c_int) -> c_int;
  pub fn tcdrain(fd: c_int) -> c_int;
  pub fn tcflush(fd: c_int, queue_selector: c_int) -> c_int;
  pub fn tcflow(fd: c_int, action: c_int) -> c_int;
  pub fn cfmakeraw(termios_p: *mut Termios) -> c_void;
  pub fn cfgetispeed(termios_p: *const Termios) -> speed_t;
  pub fn cfgetospeed(termios_p: *const Termios) -> speed_t;
  pub fn cfsetispeed(termios_p: *mut Termios, speed: speed_t) -> c_int;
  pub fn cfsetospeed(termios_p: *mut Termios, speed: speed_t) -> c_int;
  pub fn cfsetspeed(termios_p: *mut Termios, speed: speed_t) -> c_int;
}


#[cfg(target_os = "linux")]
mod os {
  use super::libc::{c_int,c_uint,c_uchar};

  #[allow(non_camel_case_types)]
  pub type cc_t = c_uchar;

  #[allow(non_camel_case_types)]
  pub type speed_t = c_uint;

  #[allow(non_camel_case_types)]
  pub type tcflag_t = c_uint;

  #[derive(Debug,Copy,Eq,PartialEq)]
  #[repr(C)]
  pub struct Termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    c_line: cc_t,
    pub c_cc: [cc_t; NCCS],
    c_ispeed: speed_t,
    c_ospeed: speed_t
  }

  pub const NCCS: usize = 32;

  // c_cc characters
  pub const VINTR:    usize = 0;
  pub const VQUIT:    usize = 1;
  pub const VERASE:   usize = 2;
  pub const VKILL:    usize = 3;
  pub const VEOF:     usize = 4;
  pub const VTIME:    usize = 5;
  pub const VMIN:     usize = 6;
  pub const VSWTC:    usize = 7;
  pub const VSTART:   usize = 8;
  pub const VSTOP:    usize = 9;
  pub const VSUSP:    usize = 10;
  pub const VEOL:     usize = 11;
  pub const VREPRINT: usize = 12;
  pub const VDISCARD: usize = 13;
  pub const VWERASE:  usize = 14;
  pub const VLNEXT:   usize = 15;
  pub const VEOL2:    usize = 16;

  // c_iflag bits
  pub const IGNBRK:  tcflag_t = 0o000001;
  pub const BRKINT:  tcflag_t = 0o000002;
  pub const IGNPAR:  tcflag_t = 0o000004;
  pub const PARMRK:  tcflag_t = 0o000010;
  pub const INPCK:   tcflag_t = 0o000020;
  pub const ISTRIP:  tcflag_t = 0o000040;
  pub const INLCR:   tcflag_t = 0o000100;
  pub const IGNCR:   tcflag_t = 0o000200;
  pub const ICRNL:   tcflag_t = 0o000400;
  pub const IUCLC:   tcflag_t = 0o001000;
  pub const IXON:    tcflag_t = 0o002000;
  pub const IXANY:   tcflag_t = 0o004000;
  pub const IXOFF:   tcflag_t = 0o010000;
  pub const IMAXBEL: tcflag_t = 0o020000;
  pub const IUTF8:   tcflag_t = 0o040000;

  // c_oflag bits
  pub const OPOST:  tcflag_t = 0o000001;
  pub const OLCUC:  tcflag_t = 0o000002;
  pub const ONLCR:  tcflag_t = 0o000004;
  pub const OCRNL:  tcflag_t = 0o000010;
  pub const ONOCR:  tcflag_t = 0o000020;
  pub const ONLRET: tcflag_t = 0o000040;
  pub const OFILL:  tcflag_t = 0o000100;
  pub const OFDEL:  tcflag_t = 0o000200;
  pub const NLDLY:  tcflag_t = 0o000400;
  pub const NL0:    tcflag_t = 0o000000;
  pub const NL1:    tcflag_t = 0o000400;
  pub const CRDLY:  tcflag_t = 0o003000;
  pub const CR0:    tcflag_t = 0o000000;
  pub const CR1:    tcflag_t = 0o001000;
  pub const CR2:    tcflag_t = 0o002000;
  pub const CR3:    tcflag_t = 0o003000;
  pub const TABDLY: tcflag_t = 0o014000;
  pub const TAB0:   tcflag_t = 0o000000;
  pub const TAB1:   tcflag_t = 0o004000;
  pub const TAB2:   tcflag_t = 0o010000;
  pub const TAB3:   tcflag_t = 0o014000;
  pub const BSDLY:  tcflag_t = 0o020000;
  pub const BS0:    tcflag_t = 0o000000;
  pub const BS1:    tcflag_t = 0o020000;
  pub const FFDLY:  tcflag_t = 0o100000;
  pub const FF0:    tcflag_t = 0o000000;
  pub const FF1:    tcflag_t = 0o100000;
  pub const VTDLY:  tcflag_t = 0o040000;
  pub const VT0:    tcflag_t = 0o000000;
  pub const VT1:    tcflag_t = 0o040000;
  pub const XTABS:  tcflag_t = 0o014000;

  // c_cflag bits
  pub const CBAUD:    tcflag_t = 0o010017;
  pub const CSIZE:    tcflag_t = 0o000060;
  pub const CS5:      tcflag_t = 0o000000;
  pub const CS6:      tcflag_t = 0o000020;
  pub const CS7:      tcflag_t = 0o000040;
  pub const CS8:      tcflag_t = 0o000060;
  pub const CSTOPB:   tcflag_t = 0o000100;
  pub const CREAD:    tcflag_t = 0o000200;
  pub const PARENB:   tcflag_t = 0o000400;
  pub const PARODD:   tcflag_t = 0o001000;
  pub const HUPCL:    tcflag_t = 0o002000;
  pub const CLOCAL:   tcflag_t = 0o004000;
  pub const CBAUDEX:  tcflag_t = 0o010000;
  pub const CIBAUD:   tcflag_t = 0o02003600000;
  pub const CMSPAR:   tcflag_t = 0o10000000000;
  pub const CRTSCTS:  tcflag_t = 0o20000000000;

  // c_lflag bits
  pub const ISIG:    tcflag_t = 0o000001;
  pub const ICANON:  tcflag_t = 0o000002;
  pub const XCASE:   tcflag_t = 0o000004;
  pub const ECHO:    tcflag_t = 0o000010;
  pub const ECHOE:   tcflag_t = 0o000020;
  pub const ECHOK:   tcflag_t = 0o000040;
  pub const ECHONL:  tcflag_t = 0o000100;
  pub const NOFLSH:  tcflag_t = 0o000200;
  pub const TOSTOP:  tcflag_t = 0o000400;
  pub const ECHOCTL: tcflag_t = 0o001000;
  pub const ECHOPRT: tcflag_t = 0o002000;
  pub const ECHOKE:  tcflag_t = 0o004000;
  pub const FLUSHO:  tcflag_t = 0o010000;
  pub const PENDIN:  tcflag_t = 0o040000;
  pub const IEXTEN:  tcflag_t = 0o100000;
  pub const EXTPROC: tcflag_t = 0o200000;

  // baud rates
  pub const B0:       speed_t = 0o000000;
  pub const B50:      speed_t = 0o000001;
  pub const B75:      speed_t = 0o000002;
  pub const B110:     speed_t = 0o000003;
  pub const B134:     speed_t = 0o000004;
  pub const B150:     speed_t = 0o000005;
  pub const B200:     speed_t = 0o000006;
  pub const B300:     speed_t = 0o000007;
  pub const B600:     speed_t = 0o000010;
  pub const B1200:    speed_t = 0o000011;
  pub const B1800:    speed_t = 0o000012;
  pub const B2400:    speed_t = 0o000013;
  pub const B4800:    speed_t = 0o000014;
  pub const B9600:    speed_t = 0o000015;
  pub const B19200:   speed_t = 0o000016;
  pub const B38400:   speed_t = 0o000017;
  pub const EXTA:     speed_t = B19200;
  pub const EXTB:     speed_t = B38400;
  pub const B57600:   speed_t = 0o010001;
  pub const B115200:  speed_t = 0o010002;
  pub const B230400:  speed_t = 0o010003;
  pub const B460800:  speed_t = 0o010004;
  pub const B500000:  speed_t = 0o010005;
  pub const B576000:  speed_t = 0o010006;
  pub const B921600:  speed_t = 0o010007;
  pub const B1000000: speed_t = 0o010010;
  pub const B1152000: speed_t = 0o010011;
  pub const B1500000: speed_t = 0o010012;
  pub const B2000000: speed_t = 0o010013;
  pub const B2500000: speed_t = 0o010014;
  pub const B3000000: speed_t = 0o010015;
  pub const B3500000: speed_t = 0o010016;
  pub const B4000000: speed_t = 0o010017;

  // tcflow()
  pub const TCOOFF: c_int = 0;
  pub const TCOON:  c_int = 1;
  pub const TCIOFF: c_int = 2;
  pub const TCION:  c_int = 3;

  // tcflush()
  pub const TCIFLUSH:  c_int = 0;
  pub const TCOFLUSH:  c_int = 1;
  pub const TCIOFLUSH: c_int = 2;

  // tcsetattr()
  pub const TCSANOW:   c_int = 0;
  pub const TCSADRAIN: c_int = 1;
  pub const TCSAFLUSH: c_int = 2;
}


#[cfg(target_os = "macos")]
mod os {
  use super::libc::{c_int,c_uchar,c_ulong};

  #[allow(non_camel_case_types)]
  pub type tcflag_t = c_ulong;

  #[allow(non_camel_case_types)]
  pub type cc_t = c_uchar;

  #[allow(non_camel_case_types)]
  pub type speed_t = c_ulong;

  #[derive(Debug,Copy,Eq,PartialEq)]
  #[repr(C)]
  pub struct Termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_cc: [cc_t; NCCS],
    c_ispeed: speed_t,
    c_ospeed: speed_t
  }

  pub const NCCS: usize = 20;

  // c_cc characters
  pub const VEOF:     usize = 0;
  pub const VEOL:     usize = 1;
  pub const VEOL2:    usize = 2;
  pub const VERASE:   usize = 3;
  pub const VWERASE:  usize = 4;
  pub const VKILL:    usize = 5;
  pub const VREPRINT: usize = 6;
  pub const VINTR:    usize = 8;
  pub const VQUIT:    usize = 9;
  pub const VSUSP:    usize = 10;
  pub const VDSUSP:   usize = 11;
  pub const VSTART:   usize = 12;
  pub const VSTOP:    usize = 13;
  pub const VLNEXT:   usize = 14;
  pub const VDISCARD: usize = 15;
  pub const VMIN:     usize = 16;
  pub const VTIME:    usize = 17;
  pub const VSTATUS:  usize = 18;

  // c_iflag bits
  pub const IGNBRK:  tcflag_t = 0x00000001;
  pub const BRKINT:  tcflag_t = 0x00000002;
  pub const IGNPAR:  tcflag_t = 0x00000004;
  pub const PARMRK:  tcflag_t = 0x00000008;
  pub const INPCK:   tcflag_t = 0x00000010;
  pub const ISTRIP:  tcflag_t = 0x00000020;
  pub const INLCR:   tcflag_t = 0x00000040;
  pub const IGNCR:   tcflag_t = 0x00000080;
  pub const ICRNL:   tcflag_t = 0x00000100;
  pub const IXON:    tcflag_t = 0x00000200;
  pub const IXOFF:   tcflag_t = 0x00000400;
  pub const IXANY:   tcflag_t = 0x00000800;
  pub const IMAXBEL: tcflag_t = 0x00002000;
  pub const IUTF8:   tcflag_t = 0x00004000;

  // c_oflag bits
  pub const OPOST:  tcflag_t = 0x00000001;
  pub const ONLCR:  tcflag_t = 0x00000002;
  pub const OXTABS: tcflag_t = 0x00000004;
  pub const ONOEOT: tcflag_t = 0x00000008;
  pub const OCRNL:  tcflag_t = 0x00000010;
  pub const ONOCR:  tcflag_t = 0x00000020;
  pub const ONLRET: tcflag_t = 0x00000040;
  pub const OFILL:  tcflag_t = 0x00000080;
  pub const NLDLY:  tcflag_t = 0x00000300;
  pub const TABDLY: tcflag_t = 0x00000c04;
  pub const CRDLY:  tcflag_t = 0x00003000;
  pub const FFDLY:  tcflag_t = 0x00004000;
  pub const BSDLY:  tcflag_t = 0x00008000;
  pub const VTDLY:  tcflag_t = 0x00010000;
  pub const OFDEL:  tcflag_t = 0x00020000;
  pub const NL0:    tcflag_t = 0x00000000;
  pub const NL1:    tcflag_t = 0x00000100;
  pub const NL2:    tcflag_t = 0x00000200;
  pub const NL3:    tcflag_t = 0x00000300;
  pub const TAB0:   tcflag_t = 0x00000000;
  pub const TAB1:   tcflag_t = 0x00000400;
  pub const TAB2:   tcflag_t = 0x00000800;
  pub const TAB3:   tcflag_t = 0x00000004;
  pub const CR0:    tcflag_t = 0x00000000;
  pub const CR1:    tcflag_t = 0x00001000;
  pub const CR2:    tcflag_t = 0x00002000;
  pub const CR3:    tcflag_t = 0x00003000;
  pub const FF0:    tcflag_t = 0x00000000;
  pub const FF1:    tcflag_t = 0x00004000;
  pub const BS0:    tcflag_t = 0x00000000;
  pub const BS1:    tcflag_t = 0x00008000;
  pub const VT0:    tcflag_t = 0x00000000;
  pub const VT1:    tcflag_t = 0x00010000;

  // c_cflag bits
  pub const CIGNORE:    tcflag_t = 0x00000001;
  pub const CSIZE:      tcflag_t = 0x00000300;
  pub const CS5:        tcflag_t = 0x00000000;
  pub const CS6:        tcflag_t = 0x00000100;
  pub const CS7:        tcflag_t = 0x00000200;
  pub const CS8:        tcflag_t = 0x00000300;
  pub const CSTOPB:     tcflag_t = 0x00000400;
  pub const CREAD:      tcflag_t = 0x00000800;
  pub const PARENB:     tcflag_t = 0x00001000;
  pub const PARODD:     tcflag_t = 0x00002000;
  pub const HUPCL:      tcflag_t = 0x00004000;
  pub const CLOCAL:     tcflag_t = 0x00008000;
  pub const CCTS_OFLOW: tcflag_t = 0x00010000;
  pub const CRTSCTS:    tcflag_t = CCTS_OFLOW | CRTS_IFLOW;
  pub const CRTS_IFLOW: tcflag_t = 0x00020000;
  pub const CDTR_IFLOW: tcflag_t = 0x00040000;
  pub const CDSR_OFLOW: tcflag_t = 0x00080000;
  pub const CCAR_OFLOW: tcflag_t = 0x00100000;
  pub const MDMBUF:     tcflag_t = 0x00100000;

  // c_lflag bits
  pub const ECHOKE:     tcflag_t = 0x00000001;
  pub const ECHOE:      tcflag_t = 0x00000002;
  pub const ECHOK:      tcflag_t = 0x00000004;
  pub const ECHO:       tcflag_t = 0x00000008;
  pub const ECHONL:     tcflag_t = 0x00000010;
  pub const ECHOPRT:    tcflag_t = 0x00000020;
  pub const ECHOCTL:    tcflag_t = 0x00000040;
  pub const ISIG:       tcflag_t = 0x00000080;
  pub const ICANON:     tcflag_t = 0x00000100;
  pub const ALTWERASE:  tcflag_t = 0x00000200;
  pub const IEXTEN:     tcflag_t = 0x00000400;
  pub const EXTPROC:    tcflag_t = 0x00000800;
  pub const TOSTOP:     tcflag_t = 0x00400000;
  pub const FLUSHO:     tcflag_t = 0x00800000;
  pub const NOKERNINFO: tcflag_t = 0x02000000;
  pub const PENDIN:     tcflag_t = 0x20000000;
  pub const NOFLSH:     tcflag_t = 0x80000000;

  // baud speeds
  pub const B0:      speed_t = 0;
  pub const B50:     speed_t = 50;
  pub const B75:     speed_t = 75;
  pub const B110:    speed_t = 110;
  pub const B134:    speed_t = 134;
  pub const B150:    speed_t = 150;
  pub const B200:    speed_t = 200;
  pub const B300:    speed_t = 300;
  pub const B600:    speed_t = 600;
  pub const B1200:   speed_t = 1200;
  pub const B1800:   speed_t = 1800;
  pub const B2400:   speed_t = 2400;
  pub const B4800:   speed_t = 4800;
  pub const B9600:   speed_t = 9600;
  pub const B19200:  speed_t = 19200;
  pub const B38400:  speed_t = 38400;
  pub const B7200:   speed_t = 7200;
  pub const B14400:  speed_t = 14400;
  pub const B28800:  speed_t = 28800;
  pub const B57600:  speed_t = 57600;
  pub const B76800:  speed_t = 76800;
  pub const B115200: speed_t = 115200;
  pub const B230400: speed_t = 230400;
  pub const EXTA:    speed_t = 19200;
  pub const EXTB:    speed_t = 38400;

  // tcflow()
  pub const TCOOFF:    c_int = 1;
  pub const TCOON:     c_int = 2;
  pub const TCIOFF:    c_int = 3;
  pub const TCION:     c_int = 4;

  // tcflush()
  pub const TCIFLUSH:  c_int = 1;
  pub const TCOFLUSH:  c_int = 2;
  pub const TCIOFLUSH: c_int = 3;

  // tcsetattr()
  pub const TCSANOW:   c_int = 0;
  pub const TCSADRAIN: c_int = 1;
  pub const TCSAFLUSH: c_int = 2;
  pub const TCSASOFT:  c_int = 0x10;
}
