# Termios Rust Bindings

The `termios` crate provides safe bindings for the Rust programming language to the [terminal I/O
interface](http://pubs.opengroup.org/onlinepubs/009695399/basedefs/termios.h.html) implemented by
Unix operating systems.  The safe bindings are a small wrapper around the raw C functions, which
converts integer return values to `std::io::IoResult` to indicate success or failure.  The raw C
functions are available in the `termios::ffi` module, but must be called within an `unsafe` block.

## Dependencies
In order to use the `termios` crate, you must have:

* A Rust runtime with the `libc` crate. Since the `libc` crate's interface is considered unstable,
  you must install Rust from the nightly builds or from the latest source.
* A native `libc` library that implements the `termios` API. This should be available on any recent
  version of OSX or Linux using `glibc`.

## Usage
Add `termios` as a dependency in `Cargo.toml`:

```toml
[dependencies]
termios = "0.0.2"
```

Import the `termios` crate and any symbols needed from `termios`. You will also probably need
`libc::c_int` for file descriptors and `std::io::Result` to propagate errors.

```rust
#![feature(libc)]

extern crate termios;
extern crate libc;

use std::io;

use libc::c_int;
use termios::*;

fn setup_fd(fd: c_int) -> io::Result<()> {
  let mut tios = try!(Termios::from_fd(fd));

  tios.c_iflag = IGNPAR | IGNBRK;
  tios.c_oflag = 0;
  tios.c_cflag = CS8 | CREAD | CLOCAL;
  tios.c_lflag = 0;

  try!(cfsetspeed(&mut tios, B9600));
  try!(tcsetattr(fd, TCSANOW, &tios));
  try!(tcflush(fd, TCIOFLUSH));

  Ok(());
}
```

## License
Copyright © 2015 David Cuddeback

Distributed under the [MIT License](LICENSE).
