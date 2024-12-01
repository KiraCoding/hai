use core::arch::asm;

/// Enable interrupts.
#[inline]
pub fn enable() {
    unsafe { asm!("msr daifclr, #2", options(nomem, nostack, preserves_flags)) };
}
