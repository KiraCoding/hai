#![no_std]
#![warn(clippy::all)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![feature(abi_x86_interrupt, abi_riscv_interrupt)]

pub mod interrupt;
pub mod memory;
