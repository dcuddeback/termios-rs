extern crate bindgen;

use {
    bindgen::{
        Builder,
        callbacks::{
            IntKind,
            ParseCallbacks,
        },
    },
    std::{
        env,
        path::PathBuf,
    },
};

#[derive(Debug)]
struct RewriteTypes { }

impl RewriteTypes {
    pub fn new() -> Self {
        Self { }
    }
}

impl ParseCallbacks for RewriteTypes {
    fn int_macro(&self, name: &str, _value: i64) -> Option<IntKind> {
        match name {
            // c_cc characters
            "VCHECKPT" |
                "VDISCARD" |
                "VDSUSP" |
                "VEOF" |
                "VEOL" |
                "VEOL2" |
                "VERASE" |
                "VERASE2" |
                "VINTR" |
                "VKILL" |
                "VLNEXT" |
                "VMIN" |
                "VREPRINT" |
                "VSTART" |
                "VSTATUS" |
                "VSTOP" |
                "VSUSP" |
                "VSWTC" |
                "VSWTCH" |
                "VTIME" |
                "VWERASE" |
                "VQUIT" => Some(IntKind::Custom { name: "usize", is_signed: false }),

            // c_iflag bits
            "BRKINT" |
                "DOSMODE" |
                "ICRNL" |
                "IGNBRK" |
                "IGNCR" |
                "IGNPAR" |
                "IMAXBEL" |
                "INLCR" |
                "INPCK" |
                "ISTRIP" |
                "IUCLC" |
                "IUTF8" |
                "IXANY" |
                "IXOFF" |
                "IXON" |
                "PARMRK" => Some(IntKind::Custom { name: "tcflag_t", is_signed: false }),

            // c_oflag bits
            "BS0" |
                "BS1" |
                "BSDLY" |
                "CR0" |
                "CR1" |
                "CR2" |
                "CR3" |
                "CRDLY" |
                "FF0" |
                "FF1" |
                "FFDLY" |
                "NL0" |
                "NL1" |
                "NL2" |
                "NL3" |
                "NLDLY" |
                "OCRNL" |
                "OFDEL" |
                "OFILL" |
                "OLCUC" |
                "ONLCR" |
                "ONLRET" |
                "ONOCR" |
                "ONOEOT" |
                "OPOST" |
                "OXTABS" |
                "PAGEOUT" |
                "TAB0" |
                "TAB1" |
                "TAB2" |
                "TAB3" |
                "TABDLY" |
                "WRAP" |
                "VT0" |
                "VT1" |
                "VTDLY" |
                "XTABS" => Some(IntKind::Custom { name: "tcflag_t", is_signed: false }),

                // c_cflag bits
                "CBAUD" |
                    "CBAUDEX" |
                    "CBAUDEXT" |
                    "CCAR_OFLOW" |
                    "CCTS_OFLOW" |
                    "CDSR_OFLOW" |
                    "CDTRCTS" |
                    "CDTR_IFLOW" |
                    "CHWFLOW" |
                    "CIBAUD" |
                    "CIBAUDEXT" |
                    "CIGNORE" |
                    "CLOCAL" |
                    "CMSPAR" |
                    "CREAD" |
                    "CRTSCTS" |
                    "CRTS_IFLOW" |
                    "CRTSXOFF" |
                    "CS5" |
                    "CS6" |
                    "CS7" |
                    "CS8" |
                    "CSIZE" |
                    "CSTOPB" |
                    "HUPCL" |
                    "LOBLK" |
                    "MDMBUF" |
                    "PARENB" |
                    "PAREXT" |
                    "PARODD" |
                    "RCV1EN" |
                    "XCLUDE" |
                    "XMT1EN" => Some(IntKind::Custom { name: "tcflag_t", is_signed: false }),

                // c_lflag bits
                "ALTWERASE" |
                    "DEFECHO" |
                    "ECHO" |
                    "ECHOCTL" |
                    "ECHOE" |
                    "ECHOK" |
                    "ECHOKE" |
                    "ECHONL" |
                    "ECHOPRT" |
                    "EXTPROC" |
                    "FLUSHO" |
                    "ICANON" |
                    "IEXTEN" |
                    "ISIG" |
                    "NOFLSH" |
                    "NOKERNINFO" |
                    "PENDIN" |
                    "TOSTOP" |
                    "XCASE" => Some(IntKind::Custom { name: "tcflag_t", is_signed: false }),

                // baud rates
                "B0" |
                    "B50" |
                    "B75" |
                    "B110" |
                    "B134" |
                    "B150" |
                    "B200" |
                    "B300" |
                    "B600" |
                    "B1200" |
                    "B1800" |
                    "B2400" |
                    "B4800" |
                    "B7200" |
                    "B9600" |
                    "B14400" |
                    "B19200" |
                    "B28800" |
                    "B38400" |
                    "B57600" |
                    "B76800" |
                    "B115200" |
                    "B153600" |
                    "B230400" |
                    "B307200" |
                    "B460800" |
                    "B500000" |
                    "B576000" |
                    "B921600" |
                    "B1000000" |
                    "B1152000" |
                    "B1500000" |
                    "B2000000" |
                    "B2500000" |
                    "B3000000" |
                    "B3500000" |
                    "B4000000" |
                    "EXTA" |
                    "EXTB" => Some(IntKind::Custom { name: "speed_t", is_signed: false }),

                // tcsetattr
                "TCSADRAIN" |
                    "TCSAFLUSH" |
                    "TCSANOW" |
                    "TCSASOFT" => Some(IntKind::Int),

                // tcflow
                "TCIOFF" |
                    "TCION" |
                    "TCOOFF" |
                    "TCOON" => Some(IntKind::Int),

                // tcflush
                "TCIFLUSH" |
                    "TCIOFLUSH" |
                    "TCOFLUSH" => Some(IntKind::Int),

            _ => None,
        }
    }
}


fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    let parse_callbacks = RewriteTypes::new();
    let mut builder = Builder::default();

    if cfg!(target_os = "illumos") {
    builder = builder
        .header("illumos.h");
    }

    let bindings = builder
        .header("wrapper.h")
        .derive_eq(true)
        .derive_partialeq(true)
        .parse_callbacks(Box::new(parse_callbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
