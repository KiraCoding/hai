#![no_std]
#![no_main]

#[uefi::entry]
#[cfg(target_os = "uefi")]
fn efi_main() -> uefi::Status {
    use uefi::boot::{exit_boot_services, MemoryType};
    use uefi::mem::memory_map::MemoryMap;
    use uefi::runtime::get_time;

    fn efi_time() -> u64 {
        let time = get_time().unwrap();
        time.hour() as u64 * 3600 + time.minute() as u64 * 60 + time.second() as u64
    }

    let time_start = efi_time();

    let mmap = unsafe { exit_boot_services(MemoryType::LOADER_DATA) };

    mmap.entries().for_each(|descriptor| match descriptor.ty {
        MemoryType::CONVENTIONAL => {
            let size = descriptor.page_count * 0x1000;
            let _end = descriptor.phys_start + size;
        }
        _ => (),
    });

    let time = efi_time() - time_start;

    kernel_main(BootInfo {
        kind: Kind::Uefi { time },
    });

    uefi::Status::SUCCESS
}

#[inline(never)]
fn kernel_main(info: BootInfo) {
    // TODO: setup idt before enabling interrupts
    // cpu64::interrupt::enable();

    loop {}
}

pub struct BootInfo {
    kind: Kind,
}

pub enum Kind {
    Uefi { time: u64 },
}
