extern crate input;
use input::{Libinput, LibinputInterface};
use std::fs::{File, OpenOptions};
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::{RawFd, FromRawFd, IntoRawFd};
use std::path::Path;
 
extern crate libc;
use libc::{O_RDONLY, O_RDWR, O_WRONLY};
 
struct Interface;
 
impl LibinputInterface for Interface {
	fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<RawFd, i32> {
		OpenOptions::new()
			.open(path)
			.map(|file| file.into_raw_fd())
			.map_err(|err| err.raw_os_error().unwrap())
	}
	fn close_restricted(&mut self, fd: RawFd) {
		unsafe {
			File::from_raw_fd(fd);
		}
	}
}
 
fn main() {
	let mut input = Libinput::new_with_udev(Interface);
	input.udev_assign_seat("seat0").unwrap();
	loop {
		input.dispatch().unwrap();
		for event in &mut input {
			println!("Got event: {:?}", event);
		}
	}
}
