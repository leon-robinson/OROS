#![no_main]
#![no_std]

use uefi::prelude::*;
use uefi_services::println;

#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    system_table.stdout().clear().unwrap();
    println!("Hello World!");

    system_table.boot_services().stall(usize::max_value());

    Status::SUCCESS
}
