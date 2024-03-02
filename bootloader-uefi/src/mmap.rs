use uefi::{
    table::{
        boot::{MemoryMap, MemoryType},
        Boot, SystemTable,
    },
    Result,
};

#[inline]
pub fn mmap(system_table: &SystemTable<Boot>) -> Result<MemoryMap> {
    let boot_services = system_table.boot_services();
    let mmap_sizes = boot_services.memory_map_size();
    let mmap_alloc_size = mmap_sizes.map_size + mmap_sizes.entry_size; // + entry_size just incase.
    let mmap_buffer = boot_services
        .allocate_pool(MemoryType::LOADER_DATA, mmap_alloc_size)
        .unwrap();

    let mmap_buffer_as_slice;
    unsafe {
        mmap_buffer_as_slice = core::slice::from_raw_parts_mut(mmap_buffer, mmap_alloc_size);
    }

    boot_services.memory_map(mmap_buffer_as_slice)
}
