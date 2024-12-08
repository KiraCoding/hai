use crate::address::VirtualAddress;

#[repr(C, align(4096))]
#[derive(Debug)]
pub struct PageTable {
    entries: [u64; 512],
}

pub fn active(offset: VirtualAddress) {
    #[cfg(target_arch = "aarch64")]
    {}

    #[cfg(target_arch = "riscv64")]
    {}

    #[cfg(target_arch = "x86_64")]
    {
        let t = x86_64::register::CR3::read_raw();
    }
}
