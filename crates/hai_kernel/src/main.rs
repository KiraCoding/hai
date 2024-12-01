#![no_std]
#![no_main]
#![warn(clippy::all)]
#![warn(clippy::nursery)]

mod paging;

#[uefi::entry]
#[cfg(target_os = "uefi")]
fn efi_main() -> uefi::Status {
    use uefi::boot::{exit_boot_services, MemoryType};

    let _mmap = unsafe { exit_boot_services(MemoryType::CONVENTIONAL) };

    kernel_main();

    uefi::Status::SUCCESS
}

#[inline(never)]
fn kernel_main() {
    loop {}
}