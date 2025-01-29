#![no_std]
#![no_main]

#[uefi::entry]
#[cfg(target_os = "uefi")]
fn efi_main() -> uefi::Status {
    use uefi::boot::{exit_boot_services, MemoryType};
    use uefi::mem::memory_map::MemoryMap;

    let mmap = unsafe { exit_boot_services(MemoryType::LOADER_DATA) };

    mmap.entries().for_each(|descriptor| match descriptor.ty {
        MemoryType::CONVENTIONAL => {
            let size = descriptor.page_count * 0x1000;
            let _end = descriptor.phys_start + size;
        }
        _ => (),
    });

    kernel_main();

    uefi::Status::SUCCESS
}

#[inline(never)]
fn kernel_main() {
    loop {}
}

#[repr(C)]
pub struct MemoryMap {
    entries: &'static [MemoryEntry],
}

#[repr(C)]
pub struct MemoryEntry {
    start: u64,
    size: u64,
}
