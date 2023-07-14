extern crate termios_sys;

pub use ::os::common::*;

pub use self::termios_sys::{
    termios,
    NCCS,

    // c_cc characters
    VWERASE,
    VREPRINT,
    VLNEXT,
    VDISCARD,
    VSTATUS,

    // c_iflag bits
    IMAXBEL,

    // c_oflag bits
    OXTABS,
    ONOEOT,

    // c_cflag bits
    CIGNORE,
    CRTSCTS,
    CRTS_IFLOW,
    CCTS_OFLOW,
    CDTRCTS,
    MDMBUF,
    CHWFLOW,

    // c_lflag bits
    ALTWERASE,
    EXTPROC,
    FLUSHO,
    NOKERNINFO,
    PENDIN,

    // baud rates
    B7200,
    B14400,
    B28800,
    B57600,
    B76800,
    B115200,
    B230400,
    B460800,
    B921600,
    EXTA,
    EXTB,

    // tcsetattr()
    TCSASOFT,
};
