#![no_std]
#![no_main]

#[uefi::entry]
#[cfg(target_os = "uefi")]
fn efi_main() -> uefi::Status {
    use core::slice::from_raw_parts;

    use uefi::boot::{exit_boot_services, image_handle, open_protocol_exclusive, MemoryType};
    use uefi::mem::memory_map::MemoryMap;
    use uefi::proto::console::gop::GraphicsOutput;
    use uefi::runtime::get_time;

    fn efi_time() -> u64 {
        let time = get_time().unwrap();
        time.hour() as u64 * 3600 + time.minute() as u64 * 60 + time.second() as u64
    }

    let time_start = efi_time();

    let _frame_buffer = {
        let mut gop = open_protocol_exclusive::<GraphicsOutput>(image_handle()).unwrap();
        let _mode = gop.current_mode_info();
        let mut frame_buffer = gop.frame_buffer();
        let data = frame_buffer.as_mut_ptr();
        let size = frame_buffer.size(); // this is bytes not len but slice from raw parts needs len

        unsafe { from_raw_parts(data, len) }
    };

    let mmap = unsafe { exit_boot_services(MemoryType::LOADER_DATA) };

    mmap.entries().for_each(|descriptor| match descriptor.ty {
        MemoryType::CONVENTIONAL => {
            let size = descriptor.page_count * 0x1000;
            let _end = descriptor.phys_start + size;
        }
        _ => (),
    });

    let efi_time = efi_time() - time_start;

    kernel_main(BootInfo {
        efi_time,
        loader: Loader::Uefi { frame_buffer: None },
    });

    uefi::Status::SUCCESS
}

#[inline(never)]
fn kernel_main(_info: BootInfo) {
    // TODO: setup idt before enabling interrupts
    // cpu64::interrupt::enable();

    loop {}
}

pub struct BootInfo {
    efi_time: u64,
    loader: Loader,
}

pub enum Loader {
    Uefi { frame_buffer: Option<u64> },
}

pub struct MemoryMap {
    entries: &'static [MemoryEntry],
}

pub struct MemoryEntry {
    start: u64,
    size: u64,
}
