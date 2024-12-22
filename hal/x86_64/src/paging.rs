use crate::register::CR0;

pub fn enable() {}

#[inline]
pub fn disable() {
    unsafe { CR0::update(|cr0| *cr0 &= !CR0::PG) };
}
