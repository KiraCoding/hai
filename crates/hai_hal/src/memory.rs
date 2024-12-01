pub struct MemoryMap {}

#[cfg(all(target_os = "uefi", feature = "uefi"))]
impl From<uefi::mem::memory_map::MemoryMapOwned> for MemoryMap {
    fn from(_value: uefi::mem::memory_map::MemoryMapOwned) -> Self {
        todo!()
    }
}
