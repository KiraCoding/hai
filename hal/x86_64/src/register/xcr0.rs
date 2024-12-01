use bitflags::bitflags;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug)]
    pub struct XCR0: u64 {}
}

impl XCR0 {}