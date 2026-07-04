use crate::expr::ValType;

struct Reg(usize);

enum Inst {
    LoadImm(Reg, ValType),
    Add(Reg, Reg, Reg),
    AddImm(Reg, Reg, ValType),
    Sub(Reg, Reg, Reg),
    And(Reg, Reg, Reg),
    Or(Reg, Reg, Reg),
    Xor(Reg, Reg, Reg),
    XorImm(Reg, Reg, ValType),
    Load(Reg, Reg, ValType),
    Store(Reg, ValType, Reg),
    Branch(Reg, ValType),
}

fn li_(dst: Reg, imm: ValType) -> Inst {
    Inst::LoadImm(dst, imm)
}

fn add_(dst: Reg, src0: Reg, src1: Reg) -> Inst {
    Inst::Add(dst, src0, src1)
}

fn addi_(dst: Reg, src: Reg, imm: ValType) -> Inst {
    Inst::AddImm(dst, src, imm)
}

fn sub_(dst: Reg, src0: Reg, src1: Reg) -> Inst {
    Inst::Sub(dst, src0, src1)
}

fn subi_(dst: Reg, src: Reg, imm: ValType) -> Inst {
    Inst::AddImm(dst, src, -imm)
}

fn and_(dst: Reg, src0: Reg, src1: Reg) -> Inst {
    Inst::And(dst, src0, src1)
}

fn or_(dst: Reg, src0: Reg, src1: Reg) -> Inst {
    Inst::Or(dst, src0, src1)
}

fn xor_(dst: Reg, src0: Reg, src1: Reg) -> Inst {
    Inst::Xor(dst, src0, src1)
}

fn xori_(dst: Reg, src: Reg, imm: ValType) -> Inst {
    Inst::XorImm(dst, src, imm)
}

fn not_(dst: Reg, src: Reg, imm: ValType) -> Inst {
    Inst::XorImm(dst, src, -1)
}

fn ld_(dst: Reg, src: Reg, imm: ValType) -> Inst {
    Inst::Load(dst, src, imm)
}

fn st_(addr: Reg, imm: ValType, val: Reg) -> Inst {
    Inst::Store(addr, imm, val)
}

fn beq_(reg: Reg) -> Inst {
    Inst::Branch(reg, -1)
}

fn bge_(reg: Reg) -> Inst {
    Inst::Branch(reg, 1 << ValType::BITS)
}

fn jmp_(reg: Reg) -> Inst {
    Inst::Branch(reg, 0)
}
