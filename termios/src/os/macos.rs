extern crate termios_sys;

pub use ::os::common::*;

pub use self::termios_sys::{
    NCCS,

    // c_cc characters
    VWERASE,
    VREPRINT,
    VDSUSP,
    VLNEXT,
    VDISCARD,
    VSTATUS,

    // c_iflag bits
    IMAXBEL,
    IUTF8,

    // c_oflag bits
    OXTABS,
    ONOEOT,
    OFILL,
    NLDLY,
    TABDLY,
    CRDLY,
    FFDLY,
    BSDLY,
    VTDLY,
    OFDEL,
    NL0,
    NL1,
    NL2,
    NL3,
    TAB1,
    TAB2,
    TAB3,
    CR0,
    CR1,
    CR2,
    CR3,
    FF0,
    FF1,
    BS0,
    BS1,
    VT0,
    VT1,

    // c_cflag bits
    CIGNORE,
    CCTS_OFLOW,
    CRTSCTS,
    CRTS_IFLOW,
    CDTR_IFLOW,
    CDSR_OFLOW,
    CCAR_OFLOW,
    MDMBUF,

    // c_lflag bits
    ALTWERASE,
    EXTPROC,
    FLUSHO,
    NOKERNINFO,
    PENDIN,

    // baud speeds
    B7200,
    B14400,
    B28800,
    B57600,
    B76800,
    B115200,
    B230400,
    EXTA,
    EXTB,

    // tcsetattr()
    TCSASOFT,
};
