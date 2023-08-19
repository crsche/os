#![feature(let_chains, trivial_bounds, iter_collect_into)]

use core::time::Duration;
use std::{env, path::PathBuf};

use bootloader::DiskImageBuilder;
fn main() {
	// IMAGE CREATION
	let kernel_path = env::var("CARGO_BIN_FILE_KERNEL").unwrap();
	let disk_builder = DiskImageBuilder::new(PathBuf::from(kernel_path));
	// specify output paths
	let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
	let bios_path = out_dir.join("blog_os-bios.img");
	// create the disk images
	disk_builder.create_bios_image(&bios_path).unwrap();
	println!("cargo:rustc-env=BIOS_PATH={}", bios_path.display());
}
