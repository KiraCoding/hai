use core::arch::asm;

/// Enable interrupts.
#[inline(always)]
pub fn enable() {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        asm!("msr daifclr, #2", options(nomem, nostack, preserves_flags))
    };

    #[cfg(target_arch = "riscv64")]
    unsafe {
        asm!(
            "csrsi sstatus, 0x2",
            options(nomem, nostack, preserves_flags)
        )
    };

    #[cfg(target_arch = "x86_64")]
    unsafe {
        asm!("sti", options(nomem, nostack, preserves_flags))
    };
}

/// Disable interrupts.
#[inline(always)]
pub fn disable() {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        asm!("msr daifset, #2", options(nomem, nostack, preserves_flags))
    };

    #[cfg(target_arch = "riscv64")]
    unsafe {
        asm!(
            "csrci sstatus, 0x2",
            options(nomem, nostack, preserves_flags)
        )
    };

    #[cfg(target_arch = "x86_64")]
    unsafe {
        asm!("cli", options(nomem, nostack, preserves_flags));
    };
}

/// Places the CPU into a low-power state, awaiting an interrupt.
#[inline(always)]
pub fn wait() {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        asm!("wfi", options(nomem, nostack, preserves_flags));
    }

    #[cfg(target_arch = "riscv64")]
    unsafe {
        asm!("wfi", options(nomem, nostack, preserves_flags));
    }

    #[cfg(target_arch = "x86_64")]
    unsafe {
        asm!("hlt", options(nomem, nostack, preserves_flags));
    };
}
