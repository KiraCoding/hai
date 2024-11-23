use core::arch::asm;

/// Enable interrupts.
#[inline(always)]
pub fn enable() {
    // SAFETY: This is only safe if the CPU is in a privileged mode that allows writing to the `DAIF` register.
    #[cfg(target_arch = "aarch64")]
    unsafe {
        asm!("msr daifclr, #2", options(nomem, nostack, preserves_flags))
    };

    // SAFETY: This is only safe if the code runs in supervisor mode, allowing access to the `sstatus` register.
    #[cfg(target_arch = "riscv64")]
    unsafe {
        asm!(
            "csrsi sstatus, 0x2",
            options(nomem, nostack, preserves_flags)
        )
    };

    // SAFETY: This is only safe if the code is running in ring 0 (kernel mode), where setting the interrupt flag is permitted.
    #[cfg(target_arch = "x86_64")]
    unsafe {
        asm!("sti", options(nomem, nostack, preserves_flags))
    };
}

/// Disable interrupts.
#[inline(always)]
pub fn disable() {
    // SAFETY: This is only safe if the CPU is in a privileged mode that allows writing to the `DAIF` register.
    #[cfg(target_arch = "aarch64")]
    unsafe {
        asm!("msr daifset, #2", options(nomem, nostack, preserves_flags))
    };

    // SAFETY: This is only safe if the code runs in supervisor mode, allowing access to the `sstatus` register.
    #[cfg(target_arch = "riscv64")]
    unsafe {
        asm!(
            "csrci sstatus, 0x2",
            options(nomem, nostack, preserves_flags)
        )
    };

    // SAFETY: This is only safe if the code is running in ring 0 (kernel mode), where clearing the interrupt flag is allowed.
    #[cfg(target_arch = "x86_64")]
    unsafe {
        asm!("cli", options(nomem, nostack, preserves_flags));
    };
}

/// Places the CPU into a low-power state, awaiting an interrupt.
#[inline(always)]
pub fn wait() {
    // SAFETY: This is only safe if the CPU is in a mode where waiting for an interrupt is allowed.
    #[cfg(target_arch = "aarch64")]
    unsafe {
        asm!("wfi", options(nomem, nostack, preserves_flags));
    }

    // SAFETY: This is only safe if the CPU is in a mode where waiting for an interrupt is allowed.
    #[cfg(target_arch = "riscv64")]
    unsafe {
        asm!("wfi", options(nomem, nostack, preserves_flags));
    }

    // SAFETY: This is only safe if the code is running in ring 0 (kernel mode), where halting the CPU is permitted.
    #[cfg(target_arch = "x86_64")]
    unsafe {
        asm!("hlt", options(nomem, nostack, preserves_flags));
    };
}
