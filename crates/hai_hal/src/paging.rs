#[repr(align(4096))]
pub struct PageTable([u64; 512]);

#[cfg(target_arch = "x86_64")]
use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

#[cfg(target_arch = "x86_64")]
extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
}
