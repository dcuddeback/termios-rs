extern crate termios_sys;

pub use ::os::common::*;

pub use self::termios_sys::{
    termios,
    NCCS,

    // c_cc characters
    VSWTCH,
    VDSUSP,
    VREPRINT,
    VDISCARD,
    VWERASE,
    VLNEXT,
    VSTATUS,

    // c_iflag bits
    IUCLC,
    IMAXBEL,
    DOSMODE,

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
    XTABS,
    BSDLY,
    BS0,
    BS1,
    VTDLY,
    VT0,
    VT1,
    FFDLY,
    FF0,
    FF1,
    PAGEOUT,
    WRAP,

    // c_cflag bits
    CBAUD,
    RCV1EN,
    XMT1EN,
    LOBLK,
    XCLUDE,
    CRTSXOFF,
    CRTSCTS,
    CIBAUD,
    PAREXT,
    CBAUDEXT,
    CIBAUDEXT,

    // c_lflag bits
    XCASE,
    FLUSHO,
    PENDIN,

    // baud rates
    EXTA,
    EXTB,
    B57600,
    B76800,
    B115200,
    B153600,
    B230400,
    B307200,
    B460800,
    B921600,
};
