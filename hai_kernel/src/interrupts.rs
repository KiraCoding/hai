#[inline(always)]
pub(crate) fn enable() {
    #[cfg(target_arch = "aarch64")]
    aarch64::irq::enable();

    #[cfg(target_arch = "x86_64")]
    x86_64::instructions::interrupts::enable();
}
