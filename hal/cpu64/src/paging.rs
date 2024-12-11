#[repr(C, align(4096))]
#[derive(Debug)]
pub struct PageTable {
    entries: [u64; 512],
}
