mod ast;
mod ir;
mod asm;

use ir::generateir;
use lalrpop_util::lalrpop_mod;
use std::env::args;
use std::fs::read_to_string;
use std::io::Result;
use koopa::back::KoopaGenerator;
use crate::asm::generateassembly;

// 引用 lalrpop 生成的解析器
// 因为我们刚刚创建了 sysy.lalrpop, 所以模块名是 sysy
lalrpop_mod!(sysy);

fn main() -> Result<()> {
  // 解析命令行参数
  let mut args = args();
  args.next();
  let mode = args.next().unwrap();
  let input = args.next().unwrap();
  args.next();
  let output = args.next().unwrap();

  // 读取输入文件
  let input = read_to_string(input)?;

  // 调用 lalrpop 生成的 parser 解析输入文件
  let ast = sysy::CompUnitParser::new().parse(&input).unwrap();

  // 输出解析得到的 AST
  println!("{:#?}", ast);

  // 生成 IR
  let ir = generateir(&ast).unwrap();


  match mode.as_str() {
    // Convert in-memory Koopa IR to text, and write it to output file (hello.koopa).
    "-koopa" => {
      let mut gen = KoopaGenerator::new(Vec::new());
      gen.generate_on(&ir).unwrap();
      std::fs::write(output, gen.writer()).expect("Unable to write");
    }
    "-riscv" => {
      generateassembly(&ir, &output).unwrap();
    }
    mode => {
      panic!("Unknown mode: {}", mode);
    }
  }
  
  
  Ok(())
}

