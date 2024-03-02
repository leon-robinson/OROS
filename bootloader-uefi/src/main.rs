#![no_main]
#![no_std]

use uefi::{prelude::*, table::boot::PAGE_SIZE};
use uefi_services::println;

mod mmap;

#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    system_table.stdout().clear().unwrap();

    println!("Hello World!");

    for (i, entry) in mmap::mmap(&system_table).unwrap().entries().enumerate() {
        if i >= 24 {
            break;
        }
        println!(
            "{:x}-{:x}   {:?}",
            entry.phys_start,
            entry.phys_start as usize + ((entry.page_count as usize) * PAGE_SIZE),
            entry.ty
        );
    }

    system_table.boot_services().stall(usize::max_value());

    Status::SUCCESS
}
