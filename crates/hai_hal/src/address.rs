#[repr(transparent)]
pub struct Physical(u64);

#[repr(transparent)]
pub struct Virtual(u64);

pub enum Address {
    Virtual(Virtual),
    Physical(Physical),
}