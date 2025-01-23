use core::arch::asm;
use core::marker::PhantomData;

/// An interrupt handler.
pub type Interrupt = extern "x86-interrupt" fn(_: u8);

/// A trap handler.
pub type Trap = extern "x86-interrupt" fn(_: u8, error_code: u64);

#[repr(C)]
#[derive(Debug)]
pub struct Table([Descriptor<Interrupt>; 255]);

/// An interrupt descriptor.
#[repr(C, packed)]
#[derive(Debug)]
pub struct Descriptor<F> {
    lower: u16,
    selector: u16,
    ist: u8,
    flags: u8,
    middle: u16,
    high: u32,
    reserved: u32,
    phantom: PhantomData<F>,
}

impl<F> Descriptor<F> {
    #[inline]
    pub const fn empty() -> Self {
        Self {
            lower: 0,
            selector: 0,
            ist: 0,
            flags: 0,
            middle: 0,
            high: 0,
            reserved: 0,
            phantom: PhantomData,
        }
    }
}

impl Descriptor<Interrupt> {}

impl Descriptor<Trap> {}

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
