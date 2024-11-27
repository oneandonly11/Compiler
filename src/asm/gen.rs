use koopa::ir::values::Return;
use koopa::ir::Program;
use koopa::ir::{BasicBlock,  FunctionData, ValueKind};
use koopa::ir::entities::ValueData;
use crate::asm::Context;
use std::fs::File;
use std::io::{Result, Write};

pub trait GenAsm{
    fn gen_asm(&self,context: &mut Context,file :&mut File ) -> Result<()>;
}

impl GenAsm for Program {
    fn gen_asm(&self,context: &mut Context,file :&mut File ) -> Result<()> {
        writeln!(file, "  .text")?;
        for &func in self.func_layout() {
            context.cur_func = Some(func);
            self.func(func).gen_asm(context,file)?;
        }
        Ok(())
    }
}

impl GenAsm for FunctionData{
    fn gen_asm(&self,context: &mut Context,file :&mut File ) -> Result<()> {
        writeln!(file, "  .global {}", &self.name()[1..]);
        writeln!(file, "{}:", &self.name()[1..]);
        // 遍历基本块列表
        for (&bb, node) in self.layout().bbs() {
            // 一些必要的处理
            bb.gen_asm(context,file)?;
            // 遍历指令列表
            for &inst in node.insts().keys() {
                let value_data = self.dfg().value(inst);
                value_data.gen_asm(context,file)?;
            }
        }
        Ok(())
    }
}


impl GenAsm for BasicBlock {
    fn gen_asm(&self,context: &mut Context,file :&mut File ) -> Result<()> {
        Ok(())
    }
    
}

impl GenAsm for ValueData {
    fn gen_asm(&self,context: &mut Context,file :&mut File ) -> Result<()> {
        match self.kind() {
            ValueKind::Return(rt) => {
                rt.gen_asm(context,file)?;
            }
            _ => {}
        }
        Ok(())
    }
}

impl GenAsm for Return {
    fn gen_asm(&self,context: &mut Context,file :&mut File ) -> Result<()> {
        if let Some(value) = self.value() {
            let value_data = context.program.func(context.cur_func.unwrap()).dfg().value(value);
            match value_data.kind() {
                ValueKind::Integer(num) => {
                    writeln!(file, "  li a0, {}", num.value())?;
                }
                _ => {}
            }
           
        } 
        writeln!(file, "  ret")?;
        Ok(())
    }
}