#![no_std]
#![no_main]

#[uefi::entry]
#[cfg(target_os = "uefi")]
fn efi_main() -> uefi::Status {
    use uefi::{
        boot::{exit_boot_services, MemoryType},
        helpers::init,
        mem::memory_map::MemoryMap,
    };

    init().unwrap();

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
    //! TODO: setup idt before enabling interrupts
    // cpu64::interrupt::enable();

    loop {}
}
