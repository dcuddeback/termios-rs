extern crate termios_sys;

pub use ::os::common::*;

pub use self::termios_sys::{
    termios,
    NCCS,

    // c_cc characters
    VWERASE,
    VREPRINT,
    VDSUSP,
    VLNEXT,
    VDISCARD,
    VSTATUS,
    VCHECKPT,

    // c_iflag bits
    IMAXBEL,

    // c_oflag bits
    OXTABS,
    ONOEOT,

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

    // baud rates
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
