use core::arch::asm;

#[inline(always)]
pub fn enable() {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        asm!("msr daifclr, #2", options(nostack, preserves_flags))
    };

    #[cfg(target_arch = "x86_64")]
    unsafe {
        asm!("sti", options(preserves_flags, nostack))
    };

    #[cfg(target_arch = "riscv64")]
    unsafe {
        asm!("csrsi sstatus, 0x2", options(nostack, preserves_flags))
    };
}

#[inline(always)]
pub fn disable() {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        asm!("msr daifset, #2", options(nostack, preserves_flags))
    };

    #[cfg(target_arch = "x86_64")]
    unsafe {
        asm!("cli", options(nostack, preserves_flags));
    };

    #[cfg(target_arch = "riscv64")]
    unsafe {
        asm!("csrci sstatus, 0x2", options(nostack, preserves_flags))
    };
}
