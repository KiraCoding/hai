#[repr(transparent)]
#[derive(Debug)]
pub struct VirtualAddress(u64);

impl From<u64> for VirtualAddress {
    #[inline]
    fn from(address: u64) -> Self {
        Self(address)
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct PhysicalAddress(u64);

impl From<u64> for PhysicalAddress {
    #[inline]
    fn from(address: u64) -> Self {
        Self(address)
    }
}
