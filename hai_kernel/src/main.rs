#![no_std]
#![no_main]

mod interrupt;
mod memory;
mod paging;

use memory::MemoryMap;

#[uefi::entry]
#[cfg(target_os = "uefi")]
fn efi_main() -> uefi::Status {
    use uefi::boot::{exit_boot_services, MemoryType};

    let mmap = unsafe { exit_boot_services(MemoryType::CONVENTIONAL) };

    kernel_main();

    uefi::Status::SUCCESS
}

#[inline(never)]
fn kernel_main() {
    // interrupt::enable();

    hai_hal::interrupt::enable();

    loop {}
}

pub struct BootInfo {
    pub mmap: MemoryMap,
}

impl BootInfo {
    pub fn new(mmap: impl Into<MemoryMap>) -> Self {
        Self { mmap: mmap.into() }
    }
}
