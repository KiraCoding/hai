pub struct MemoryMap {}

#[cfg(target_os = "uefi")]
impl From<uefi::mem::memory_map::MemoryMapOwned> for MemoryMap {
    fn from(_value: uefi::mem::memory_map::MemoryMapOwned) -> Self {
        todo!()
    }
}
