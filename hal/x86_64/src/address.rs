#[repr(transparent)]
#[derive(Debug)]
pub struct VirtualAddress(u64);

impl VirtualAddress {
    pub fn new(address: u64) -> Self {
        Self(address)
    }
}