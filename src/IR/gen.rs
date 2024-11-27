use koopa::ir::{Program, FunctionData, Type, builder_traits::*};
use crate::ast::*;
use crate::ir::Context;

pub trait GenerateIR {
    fn gen_ir(&self, program: &mut Program,context: &mut Context ) -> Result<(),String>;
}

impl GenerateIR for CompUnit {
    fn gen_ir(&self, program: &mut Program,context: &mut Context) -> Result<(),String> {
        self.func_def.gen_ir(program,context)?;
        Ok(())
    }
}

impl GenerateIR for FuncDef {
    fn gen_ir(&self, program: &mut Program,context: &mut Context) -> Result<(),String> {
        let func_type = match self.func_type {
            FuncType::Int => Type::get_i32(),
        };

        let func = program.new_func(
            FunctionData::new(
                format!("@{}", self.ident),
                Vec::new(),
                func_type,
            )
        );

        let func_data = program.func_mut(func);
        let new_bb = func_data.dfg_mut().new_bb().basic_block(Some("%entry".to_string()));
        func_data.layout_mut().bbs_mut().extend([new_bb]);
        context.cur_func = Some(func);
        context.cur_bb = Some(new_bb);
        self.block.gen_ir(program,context)?;
        Ok(())
    }
}

impl GenerateIR for Block {
    fn gen_ir(&self, program: &mut Program,context: &mut Context) -> Result<(),String> {
        self.stmt.gen_ir(program,context)?;
        Ok(())
    }
}

impl GenerateIR for Stmt {
    fn gen_ir(&self, program: &mut Program,context: &mut Context) -> Result<(),String> {
        let func_data = program.func_mut(context.cur_func.unwrap());
        let value = func_data.dfg_mut().new_value().integer(self.num);
        let stmt = func_data.dfg_mut().new_value().ret(Some(value));
        func_data.layout_mut().bb_mut(context.cur_bb.unwrap()).insts_mut().extend([stmt]);
        Ok(())
    }
}


