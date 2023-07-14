extern crate termios_sys;

pub use self::termios_sys::{
    // types
    cc_t,speed_t,tcflag_t,

    // c_cc subscripts
    VEOF,VEOL,VERASE,VINTR,VKILL,VMIN,VQUIT,VSTART,VSTOP,VSUSP,VTIME,

    // input modes
    BRKINT,ICRNL,IGNBRK,IGNCR,IGNPAR,INLCR,INPCK,ISTRIP,IXANY,IXOFF,IXON,PARMRK,

    // output modes
    OPOST,ONLCR,OCRNL,ONOCR,ONLRET,

    // baud rate selection
    B0,B50,B75,B110,B134,B150,B200,B300,B600,B1200,B1800,B2400,B4800,B9600,B19200,B38400,

    // control modes
    CSIZE,CS5,CS6,CS7,CS8,CSTOPB,CREAD,PARENB,PARODD,HUPCL,CLOCAL,

    // local modes
    ECHO,ECHOE,ECHOK,ECHONL,ICANON,IEXTEN,ISIG,NOFLSH,TOSTOP,

    // attribute selection
    TCSANOW,TCSADRAIN,TCSAFLUSH,

    // line control
    TCIFLUSH,TCIOFLUSH,TCOFLUSH,TCIOFF,TCION,TCOOFF,TCOON,
};
