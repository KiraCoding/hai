use core::arch::asm;

/// Enable interrupts.
#[inline(always)]
pub fn enable() {
    unsafe {
        #[cfg(target_arch = "aarch64")]
        asm!("msr daifclr, #2", options(nomem, nostack, preserves_flags));

        #[cfg(target_arch = "riscv64")]
        asm!(
            "csrsi sstatus, 0x2",
            options(nomem, nostack, preserves_flags)
        );

        #[cfg(target_arch = "x86_64")]
        asm!("sti", options(nomem, nostack, preserves_flags));
    };
}

/// Disable interrupts.
#[inline(always)]
pub fn disable() {
    unsafe {
        #[cfg(target_arch = "aarch64")]
        asm!("msr daifset, #2", options(nomem, nostack, preserves_flags));

        #[cfg(target_arch = "riscv64")]
        asm!(
            "csrci sstatus, 0x2",
            options(nomem, nostack, preserves_flags)
        );

        #[cfg(target_arch = "x86_64")]
        asm!("cli", options(nomem, nostack, preserves_flags));
    };
}

/// Places the CPU into a low-power state, awaiting an interrupt.
#[inline(always)]
pub fn wait() {
    unsafe {
        #[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
        asm!("wfi", options(nomem, nostack, preserves_flags));

        #[cfg(target_arch = "x86_64")]
        asm!("hlt", options(nomem, nostack, preserves_flags));
    }
}
