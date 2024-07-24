#![allow(non_camel_case_types)]

use libc::{c_int, c_uchar, c_uint};

pub type cc_t = c_uchar;
pub type speed_t = c_uint;
pub type tcflag_t = c_uint;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_cc: [cc_t; NCCS],
}

pub const NCCS: usize = 16;

// c_cc characters
pub const VINTR: usize = 0;
pub const VQUIT: usize = 1;
pub const VERASE: usize = 2;
pub const VKILL: usize = 3;
pub const VMIN: usize = 4;
pub const VEOF: usize = 4;
pub const VTIME: usize = 5;
pub const VEOL: usize = 5;
pub const VEOL2: usize = 6;
pub const VSTART: usize = 7;
pub const VSTOP: usize = 8;
pub const VSUSP: usize = 9;
pub const VDSUSP: usize = 10;
pub const VREPRINT: usize = 11;
pub const VDISCRD: usize = 12;
pub const VWERSE: usize = 13;
pub const VLNEXT: usize = 14;

// c_iflag bits
pub const IGNBRK: tcflag_t = 0o000001;
pub const BRKINT: tcflag_t = 0o000002;
pub const IGNPAR: tcflag_t = 0o000004;
pub const PARMRK: tcflag_t = 0o000010;
pub const INPCK: tcflag_t = 0o000020;
pub const ISTRIP: tcflag_t = 0o000040;
pub const INLCR: tcflag_t = 0o000100;
pub const IGNCR: tcflag_t = 0o000200;
pub const ICRNL: tcflag_t = 0o000400;
pub const IUCLC: tcflag_t = 0o4000;
pub const IXON: tcflag_t = 0o1000;
pub const IXANY: tcflag_t = 0o10000;
pub const IXOFF: tcflag_t = 0o2000;
pub const IMAXBEL: tcflag_t = 0o200000;

// c_oflag bits
pub const OPOST: tcflag_t = 0o000001;
pub const OLCUC: tcflag_t = 0o000002;
pub const ONLCR: tcflag_t = 0o000004;
pub const OCRNL: tcflag_t = 0o000010;
pub const ONOCR: tcflag_t = 0o000020;
pub const ONLRET: tcflag_t = 0o000040;
pub const OFILL: tcflag_t = 0o000100;
pub const OFDEL: tcflag_t = 0o000200;
pub const NLDLY: tcflag_t = 0o40000;
pub const NL0: tcflag_t = 0o00000;
pub const NL1: tcflag_t = 0o40000;
pub const CRDLY: tcflag_t = 0o1400;
pub const CR0: tcflag_t = 0o000000;
pub const CR1: tcflag_t = 0o400;
pub const CR2: tcflag_t = 0o1000;
pub const CR3: tcflag_t = 0o1400;
pub const TABDLY: tcflag_t = 0o6000;
pub const TAB0: tcflag_t = 0o0000;
pub const TAB1: tcflag_t = 0o2000;
pub const TAB2: tcflag_t = 0o4000;
pub const TAB3: tcflag_t = 0o6000;
pub const BSDLY: tcflag_t = 0o10000;
pub const BS0: tcflag_t = 0o00000;
pub const BS1: tcflag_t = 0o10000;
pub const FFDLY: tcflag_t = 0o20000;
pub const FF0: tcflag_t = 0o00000;
pub const FF1: tcflag_t = 0o20000;
pub const VTDLY: tcflag_t = 0o100000;
pub const VT0: tcflag_t = 0o000000;
pub const VT1: tcflag_t = 0o040000;
pub const XTABS: tcflag_t = 0o6000;

// c_cflag bits
pub const CBAUD: tcflag_t = 0o17;
pub const CSIZE: tcflag_t = 0o000060;
pub const CS5: tcflag_t = 0o000000;
pub const CS6: tcflag_t = 0o000020;
pub const CS7: tcflag_t = 0o000040;
pub const CS8: tcflag_t = 0o000060;
pub const CSTOPB: tcflag_t = 0o000100;
pub const CREAD: tcflag_t = 0o000200;
pub const PARENB: tcflag_t = 0o000400;
pub const PARODD: tcflag_t = 0o001000;
pub const HUPCL: tcflag_t = 0o002000;
pub const CLOCAL: tcflag_t = 0o004000;
pub const CIBAUD: tcflag_t = 0o170000;

// c_lflag bits
pub const ISIG: tcflag_t = 0o000001;
pub const ICANON: tcflag_t = 0o000002;
pub const XCASE: tcflag_t = 0o000004;
pub const ECHO: tcflag_t = 0o000010;
pub const ECHOE: tcflag_t = 0o000020;
pub const ECHOK: tcflag_t = 0o000040;
pub const ECHONL: tcflag_t = 0o000100;
pub const NOFLSH: tcflag_t = 0o000200;
pub const TOSTOP: tcflag_t = 0o200000;
pub const ECHOCTL: tcflag_t = 0o400000;
pub const ECHOPRT: tcflag_t = 0o1000000;
pub const ECHOKE: tcflag_t = 0o2000000;
pub const FLUSHO: tcflag_t = 0o4000000;
pub const PENDIN: tcflag_t = 0o4000000000;
pub const IEXTEN: tcflag_t = 0o10000000;

// baud rates
pub const B0: speed_t = 0o000000;
pub const B50: speed_t = 0o000001;
pub const B75: speed_t = 0o000002;
pub const B110: speed_t = 0o000003;
pub const B134: speed_t = 0o000004;
pub const B150: speed_t = 0o000005;
pub const B200: speed_t = 0o000006;
pub const B300: speed_t = 0o000007;
pub const B600: speed_t = 0o000010;
pub const B1200: speed_t = 0o000011;
pub const B1800: speed_t = 0o000012;
pub const B2400: speed_t = 0o000013;
pub const B4800: speed_t = 0o000014;
pub const B9600: speed_t = 0o000015;
pub const B19200: speed_t = 0o000016;
pub const B38400: speed_t = 0o000017;
pub const EXTA: speed_t = B19200;
pub const EXTB: speed_t = B38400;

// tcflow()
pub const TCOOFF: c_int = 0;
pub const TCOON: c_int = 1;
pub const TCIOFF: c_int = 2;
pub const TCION: c_int = 3;

// tcflush()
pub const TCIFLUSH: c_int = 0;
pub const TCOFLUSH: c_int = 1;
pub const TCIOFLUSH: c_int = 2;

// tcsetattr()
pub const TCSANOW: c_int = 0;
pub const TCSADRAIN: c_int = 1;
pub const TCSAFLUSH: c_int = 2;
