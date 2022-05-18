use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::any::Any;

use crate::rsp::rsp::RSP;
use crate::rsp::rsp_assembler::{E, Element, GPR, RSPAssembler, VR, VSARAccumulator};
use crate::rsp::spmem::SPMEM;
use crate::tests::{Level, Test};
use crate::tests::soft_asserts::soft_assert_eq;

fn run_test(e: Element, expected_result: [u16; 8], expected_acc_top: [u16; 8], expected_acc_mid: [u16; 8], expected_acc_low: [u16; 8]) -> Result<(), String> {
    // Prepare input data
    SPMEM::write_vector16_into_dmem(0x00, &[0x0000, 0x0001, 0x7FFF, 0x7FFF, 0x8000, 0x8000, 0xFFFE, 0xFFFF]);
    SPMEM::write_vector16_into_dmem(0x10, &[0x0000, 0x0001, 0x7FFF, 0xFFFF, 0x7FFF, 0x7FFF, 0x0001, 0x0001]);

    // Assemble RSP program
    let mut assembler = RSPAssembler::new(0);

    assembler.write_lqv(VR::V0, E::_0, 0x000, GPR::R0);
    assembler.write_lqv(VR::V1, E::_0, 0x010, GPR::R0);

    assembler.write_lqv(VR::V6, E::_0, 0x000, GPR::R0);
    assembler.write_lqv(VR::V7, E::_0, 0x010, GPR::R0);

    assembler.write_vmulq(VR::V2, VR::V0, VR::V1, e);

    assembler.write_vsar(VR::V3, VSARAccumulator::High);
    assembler.write_vsar(VR::V4, VSARAccumulator::Mid);
    assembler.write_vsar(VR::V5, VSARAccumulator::Low);

    // again but this time destructive by overwriting a source reg
    assembler.write_vmulq(VR::V6, VR::V6, VR::V1, e);
    assembler.write_vmulq(VR::V7, VR::V0, VR::V7, e);

    assembler.write_sqv(VR::V2, E::_0, 0x100, GPR::R0);
    assembler.write_sqv(VR::V3, E::_0, 0x110, GPR::R0);
    assembler.write_sqv(VR::V4, E::_0, 0x120, GPR::R0);
    assembler.write_sqv(VR::V5, E::_0, 0x130, GPR::R0);
    assembler.write_sqv(VR::V6, E::_0, 0x140, GPR::R0);
    assembler.write_sqv(VR::V7, E::_0, 0x150, GPR::R0);

    assembler.write_break();

    RSP::run_and_wait(0);

    soft_assert_eq(SPMEM::read_vector16_from_dmem(0x100), expected_result, "Result")?;
    soft_assert_eq(SPMEM::read_vector16_from_dmem(0x110), expected_acc_top, "Acc[32..48]")?;
    soft_assert_eq(SPMEM::read_vector16_from_dmem(0x120), expected_acc_mid, "Acc[16..32]")?;
    soft_assert_eq(SPMEM::read_vector16_from_dmem(0x130), expected_acc_low, "Acc[0..16]")?;
    soft_assert_eq(SPMEM::read_vector16_from_dmem(0x140), expected_result, "Result when doing VMULQ V6, V6, V1")?;
    soft_assert_eq(SPMEM::read_vector16_from_dmem(0x150), expected_result, "Result when doing VMULQ V7, V0, V7")?;

    Ok(())
}

pub struct VMULQAll {}

impl Test for VMULQAll {
    fn name(&self) -> &str { "RSP VMULQ" }

    fn level(&self) -> Level { Level::BasicFunctionality }

    fn values(&self) -> Vec<Box<dyn Any>> { Vec::new() }

    fn run(&self, _value: &Box<dyn Any>) -> Result<(), String> {
        run_test(
            Element::All,
            [0, 0, 0x7ff0, 0xc010, 0x8000, 0x8000, 0, 0],
            [0, 0, 0x3fff, 0xffff, 0xc000, 0xc000, 0, 0],
            [0, 1, 1, 0x8020, 0x801f, 0x801f, 0x1d, 0x1e],
            [0, 0, 0, 0, 0, 0, 0, 0],
        )
    }
}

pub struct VMULQH1 {}

impl Test for VMULQH1 {
    fn name(&self) -> &str { "RSP VMULQ (e=H1)" }

    fn level(&self) -> Level { Level::BasicFunctionality }

    fn values(&self) -> Vec<Box<dyn Any>> { Vec::new() }

    fn run(&self, _value: &Box<dyn Any>) -> Result<(), String> {
        run_test(
            Element::H1,
            [0, 0, 0x3ff0, 0, 0x8000, 0x8000, 0xc000, 0xc000],
            [0, 0, 0, 0, 0xc000, 0xc000, 0xffff, 0xffff],
            [0, 1, 0x7fff, 0x1e, 0x801f, 0x801f, 0x801f, 0x801f],
            [0, 0, 0, 0, 0, 0, 0, 0],
        )
    }
}
