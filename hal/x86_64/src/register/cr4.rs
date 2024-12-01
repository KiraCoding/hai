use bitflags::bitflags;
use core::arch::asm;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug)]
    pub struct CR4: u64 {
        /// Virtual-8086 Mode Extensions
        /// - If set, enables support for the virtual interrupt flag (VIF) in virtual-8086 mode.
        const VME = 1 << 0;

        /// Protected-mode Virtual Interrupts
        /// - If set, enables support for the virtual interrupt flag (VIF) in protected mode.
        const PVI = 1 << 1;

        /// Time Stamp Disable
        /// - If set, RDTSC instruction can only be executed when in ring 0,
        /// otherwise RDTSC can be used at any privilege level.
        const TSD = 1 << 2;

        /// Debugging Extensions
        /// - If set, enables debug register based breaks on I/O space access.
        const DE = 1 << 3;

        /// Page Size Extension
        /// - If set, enables 32-bit paging mode to use 4 MiB huge pages in addition to 4 KiB pages.
        /// - If PAE is enabled or the processor is in x86-64 long mode this bit is ignored.
        const PSE = 1 << 4;

        /// Physical Address Extension
        /// - If set, changes page table layout to translate 32-bit
        /// virtual addresses into extended 36-bit physical addresses.
        const PAE = 1 << 5;

        /// Machine Check Exception
        /// - If set, enables machine check interrupts to occur.
        const MCE = 1 << 6;

        /// Page Global Enabled
        /// - If set, address translations (PDE or PTE records) may be shared between address spaces.
        const PGE = 1 << 7;

        /// Performance-Monitoring Counter enable
        /// - If set, RDPMC can be executed at any privilege level, else RDPMC can only be used in ring 0.
        const PCE = 1 << 8;

        /// Operating system support for FXSAVE and FXRSTOR instructions
        /// - If set, enables Streaming SIMD Extensions (SSE) instructions and fast FPU save & restore.
        const OSFXSR = 1 << 9;

        /// Operating System Support for Unmasked SIMD Floating-Point Exceptions
        /// - If set, enables unmasked SSE exceptions.
        const OSXMMEXCPT = 1 << 10;

        /// User-Mode Instruction Prevention
        /// - If set, the SGDT, SIDT, SLDT, SMSW and STR instructions cannot be executed if CPL > 0.
        const UMIP = 1 << 11;

        /// Virtual Machine Extensions Enable
        const VMXE = 1 << 13;

        /// Safer Mode Extensions Enable
        const SMXE = 1 << 14;

        /// FSGSBASE Enable
        /// - If set, enables the instructions `RDFSBASE`, `RDGSBASE`, `WRFSBASE`, and `WRGSBASE`.
        const FSGSBASE = 1 << 16;

        /// PCID Enable
        /// - If set, enables process-context identifiers (PCIDs).
        const PCIDE = 1 << 17;

        /// XSAVE and Processor Extended States Enable
        const OSXSAVE = 1 << 18;

        /// Supervisor Mode Execution Protection Enable
        /// - If set, execution of code in a higher ring generates a fault.
        const SMEP = 1 << 20;

        /// Supervisor Mode Access Prevention Enable
        /// - If set, access of data in a higher ring generates a fault.
        const SMAP = 1 << 21;

        /// Protection Key Enable
        const PKE = 1 << 22;

        /// Control-flow Enforcement Technology
        /// - If set, enables control-flow enforcement technology.
        const CET = 1 << 23;

        /// Enable Protection Keys for Supervisor-Mode Pages
        /// - If set, each supervisor-mode linear address is associated
        /// with a protection key when 4-level or 5-level paging is in use
        const PKS = 1 << 24;
    }
}

impl CR4 {
    #[inline]
    pub unsafe fn read() -> Self {
        let cr4: u64;
        unsafe { asm!("mov {}, cr4", out(reg) cr4, options(nomem, nostack, preserves_flags)) };
        Self::from_bits_truncate(cr4)
    }

    #[inline]
    pub unsafe fn write(cr4: Self) {
        unsafe { asm!("mov cr4, {}", in(reg) cr4.bits(), options(nomem, nostack, preserves_flags)) }
    }
}
