#![no_std]
#![no_main]

use uefi::mem::memory_map::MemoryMapOwned;

#[uefi::entry]
#[cfg(target_os = "uefi")]
fn efi_main() -> uefi::Status {
    use uefi::boot::{exit_boot_services, MemoryType};

    let mmap = unsafe { exit_boot_services(MemoryType::CONVENTIONAL) };

    kernel_main(Args { mmap });

    uefi::Status::SUCCESS
}

#[inline(never)]
fn kernel_main(args: Args) {
    cpu64::interrupt::enable();

    loop {}
}

struct Args {
    mmap: MemoryMapOwned,
}
