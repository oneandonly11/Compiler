mod gen;
use crate::asm::gen::GenAsm;


use koopa::ir::{Function, Program};

pub fn generateassembly(program: &Program, path : &str) -> Result<(), String> {
    let mut context = Context {
        cur_func: None,
        program: program.clone(),
    };
    let mut file = std::fs::File::create(path).map_err(|e| format!("{:?}", e))?;
    program.gen_asm(&mut context,&mut file).map_err(|e| format!("{:?}", e))?;
    Ok(())
}

pub struct Context<'a> {
    cur_func: Option<Function>,
    program: &'a Program,
}