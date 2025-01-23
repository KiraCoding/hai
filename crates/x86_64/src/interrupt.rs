use core::arch::asm;
use core::marker::PhantomData;

/// An interrupt handler.
pub type Interrupt = fn();

/// A trap handler.
pub type Trap = fn();

/// An interrupt descriptor.
#[repr(C, packed)]
#[derive(Debug)]
pub struct Descriptor<P> {
    lower: u16,
    selector: u16,
    ist: u8,
    flags: u8,
    middle: u16,
    high: u32,
    reserved: u32,
    _phantom: PhantomData<P>,
}

impl Descriptor<Interrupt> {}

/// Enable interrupts.
#[inline]
pub fn enable() {
    unsafe { asm!("sti", options(nomem, nostack, preserves_flags)) };
}

/// Disable interrupts.
#[inline]
pub fn disable() {
    unsafe { asm!("cli", options(nomem, nostack, preserves_flags)) };
}
