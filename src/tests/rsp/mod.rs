use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::any::Any;

use crate::rsp::rsp::RSP;
use crate::tests::{Level, Test};
use crate::tests::soft_asserts::soft_assert_eq;

pub mod op_break;
pub mod wrap_around;
pub mod op_lqv_sqv;
pub mod op_vmacf;
pub mod op_vmulf;
pub mod op_vsar;
pub mod stresstests;

// Ensure that the PC reg is properly masked with 0xFFC when being written to
pub struct PCRegMasking {

}

impl Test for PCRegMasking {
    fn name(&self) -> &str { "RSP PC REG" }

    fn level(&self) -> Level { Level::BasicFunctionality }

    fn values(&self) -> Vec<Box<dyn Any>> { Vec::new() }

    fn run(&self, _value: &Box<dyn Any>) -> Result<(), String> {
        RSP::set_pc(0xFFFFFFFF);
        soft_assert_eq(RSP::pc(), 0xFFC, "RSP PC isn't masked properly on write (0xFFFFFFFF was written)")?;

        Ok(())
    }
}