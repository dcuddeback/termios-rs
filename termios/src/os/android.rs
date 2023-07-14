extern crate termios_sys;

pub use ::os::common::*;

pub use self::termios_sys::{
    termios,
    NCCS,

    // c_cc characters
    VSWTC,
    VREPRINT,
    VDISCARD,
    VWERASE,
    VLNEXT,

    // c_iflag bits
    IUCLC,
    IMAXBEL,
    IUTF8,

    // c_oflag bits
    OLCUC,
    OFILL,
    OFDEL,
    NLDLY,
    NL0,
    NL1,
    CRDLY,
    CR0,
    CR1,
    CR2,
    CR3,
    TABDLY,
    TAB1,
    TAB2,
    TAB3,
    BSDLY,
    BS0,
    BS1,
    FFDLY,
    FF0,
    FF1,
    VTDLY,
    VT0,
    VT1,
    XTABS,

    // c_cflag bits
    CBAUD,
    CBAUDEX,
    CIBAUD,
    CMSPAR,
    CRTSCTS,

    // c_lflag bits
    XCASE,
    FLUSHO,
    PENDIN,
    EXTPROC,

    // baud rates
    EXTA,
    EXTB,
    B57600,
    B115200,
    B230400,
    B460800,
    B576000,
    B921600,
    B1000000,
    B1152000,
    B2500000,
    B3500000,
    B4000000,
};
