use crate::memory::MemoryMap;

#[derive(Debug)]
pub struct FrameAlloc {
    regions: MemoryMap,
}
