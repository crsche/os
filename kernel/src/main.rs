#![no_std]
#![no_main]

#[macro_use]
extern crate log;

use core::panic::PanicInfo;

use bootloader_api::{entry_point, info::FrameBufferInfo, BootInfo};
use bootloader_x86_64_common::logger::LockedLogger;
use conquer_once::spin::OnceCell;

pub(crate) static L: OnceCell<LockedLogger> = OnceCell::uninit();

pub(crate) fn init_logger(buffer: &'static mut [u8], info: FrameBufferInfo) {
	let logger = L.get_or_init(move || LockedLogger::new(buffer, info, true, false));
	log::set_logger(logger).expect("Logger already set");
	log::set_max_level(log::LevelFilter::Trace);
	info!("Hello, Kernel Mode!");
}

entry_point!(kmain);
fn kmain(bi: &'static mut BootInfo) -> ! {
	let fb = bi.framebuffer.as_mut().unwrap();
	let fbi = fb.info();

	init_logger(fb.buffer_mut(), fbi);
	warn!("lmao this is a warning :D");

	loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { loop {} }
