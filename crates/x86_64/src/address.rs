#[repr(transparent)]
#[derive(Debug)]
pub struct VirtualAddress(u64);

impl VirtualAddress {
    #[inline]
    pub fn new(address: u64) -> Self {
        Self(address)
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct PhysicalAddress(u64);