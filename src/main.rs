mod parser;
mod runner;

use parser::Script;
use runner::Runner;

use input::{Libinput, LibinputInterface};
use std::fs::{File, OpenOptions};
use std::os::unix::{fs::OpenOptionsExt, io::OwnedFd};
use std::path::Path;

extern crate libc;
use libc::{O_RDONLY, O_RDWR, O_WRONLY};

struct Interface;

impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<OwnedFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .read((flags & O_RDONLY != 0) | (flags & O_RDWR != 0))
            .write((flags & O_WRONLY != 0) | (flags & O_RDWR != 0))
            .open(path)
            .map(|file| file.into())
            .map_err(|err| err.raw_os_error().unwrap())
    }

    fn close_restricted(&mut self, fd: OwnedFd) {
        drop(File::from(fd));
    }
}

fn main() {
    let script_path = std::env::args().skip(1).next().expect("No path passed.");

    println!("{}", script_path);

    let script = Script::read(script_path);

    let mut input = Libinput::new_with_udev(Interface);
    input.udev_assign_seat("seat0").unwrap();

    let mut runner = Runner::new(input, script);

    runner.run();
}
