use core_mir::{Mir, Load, Reg, BinOpType, PreOpType};
use impl_pass_mir::MirDigest;

pub fn interpret(digest: MirDigest) {
    let mut mem = vec![0_i64; digest.max_reg_count];

    let blocks = &digest.blocks;
    let mut current_block = blocks[0].as_ref().unwrap().mir.iter();
    
    loop {
        let mir = match current_block.next() {
            Some(mir) => mir,
            None => return,
        };
        
        match *mir {
            Mir::Print(Reg(reg)) => println!("{}", mem[reg]),
            Mir::Jump(target) => current_block = blocks[target].as_ref().unwrap().mir.iter(),
            Mir::BranchTrue { ref cond, target } => if mem[cond.0] != 0 {
                current_block = blocks[target].as_ref().unwrap().mir.iter()
            }
            Mir::Load { to: Reg(to), from } => match from {
                Load::Bool(x) => mem[to] = x as _,
                Load::U8(x) => mem[to] = x as _,
                Load::U16(x) => mem[to] = x as _,
                Load::U32(x) => mem[to] = x as _,
                Load::U64(_) => panic!("cannot load 64-bit literals!"),
                Load::U128(_) => panic!("cannot load 128-bit literals!"),
            }
            Mir::LoadReg { to: Reg(to), from: Reg(from) } => 
                mem[to] = mem[from],
            Mir::BinOp { op, out: Reg(to), left: Reg(left), right: Reg(right) } => {
                let left = mem[left];
                let right = mem[right];

                mem[to] = match op {
                    BinOpType::Add => left + right,
                    BinOpType::Sub => left - right,
                    BinOpType::Mul => left * right,
                    BinOpType::Div => left / right,
                    
                    BinOpType::Equal => (left == right) as _,
                    BinOpType::NotEqual => (left != right) as _,
                    BinOpType::LessThan => (left < right) as _,
                    BinOpType::GreaterThan => (left > right) as _,
                    BinOpType::LessThanOrEqual => (left <= right) as _,
                    BinOpType::GreaterThanOrEqual => (left >= right) as _,
                };
            }
            Mir::PreOp { op, out: Reg(to), arg: Reg(arg) } => {
                let arg = mem[arg];

                mem[to] = match op {
                    PreOpType::Not => !arg,
                    PreOpType::Neg => -arg,
                };
            }
        }
    }
}