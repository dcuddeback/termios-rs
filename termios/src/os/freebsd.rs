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

    // tcsetattr()
    TCSASOFT,
};
