mod gen;

use crate::ast::*;
use crate::ir::gen::GenerateIR;
use koopa::ir::{Program};
use koopa::ir::entities::{BasicBlock, Function};

pub fn generateir(comp_unit: &CompUnit) -> Result<Program,String> {
    let mut program = Program::new();
    let mut context = Context {
        cur_func: None,
        cur_bb: None,
    };
    comp_unit.gen_ir(&mut program, &mut context).map_err(|e| format!("{:?}", e))?;
    Ok(program)
}

pub struct Context {
    pub cur_func: Option<Function>,
    pub cur_bb: Option<BasicBlock>,
}