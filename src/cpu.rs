use crate::instruction::{Instruction, InstructionType};
use crate::memory::Memory;
use crate::registers::Registers;

pub struct CPU {
    memory: Memory,
    pc: usize,
    registers: Registers,
    pub(crate) halted: bool,
}

impl CPU {
    pub fn from_memory(memory: &Memory) -> Self {
        Self { memory: memory.clone(), pc: 0, registers: Registers::new(), halted: false }
    }

    pub fn dump_memory(&self) {
        self.memory.dump()
    }

    pub fn dump_registers(&self) {
        self.registers.dump();
        println!(" pc  {:08x}", self.pc);
    }

    pub fn run(&mut self) {
        while self.pc < self.memory.len() - 4 && !self.halted {
            self.tick()
        }
    }

    pub fn tick(&mut self) {
        let instruction = Instruction::from_u32(self.memory.get32(self.pc));
        println!("{:08x}    {}", self.pc, instruction);
        self.execute_instruction(&instruction);
    }

    pub fn execute_instruction(&mut self, instruction: &Instruction) {
        match instruction._type() {
            Ok(_type) => match _type {
                InstructionType::LUI => self.execute_lui(instruction),
                InstructionType::AUIPC => self.execute_auipc(instruction),
                InstructionType::JAL => self.execute_jal(instruction),
                InstructionType::JALR => self.execute_jalr(instruction),
                InstructionType::BEQ => self.execute_beq(instruction),
                InstructionType::BNE => self.execute_bne(instruction),
                InstructionType::BLT => self.execute_blt(instruction),
                InstructionType::BGE => self.execute_bge(instruction),
                InstructionType::BLTU => self.execute_bltu(instruction),
                InstructionType::BGEU => self.execute_bgeu(instruction),
                InstructionType::LB => self.execute_lb(instruction),
                InstructionType::LH => self.execute_lh(instruction),
                InstructionType::LW => self.execute_lw(instruction),
                InstructionType::LBU => self.execute_lbu(instruction),
                InstructionType::LHU => self.execute_lhu(instruction),
                InstructionType::SB => self.execute_sb(instruction),
                InstructionType::SH => self.execute_sh(instruction),
                InstructionType::SW => self.execute_sw(instruction),
                InstructionType::ADDI => self.execute_addi(instruction),
                InstructionType::SLTI => self.execute_slti(instruction),
                InstructionType::SLTIU => self.execute_sltiu(instruction),
                InstructionType::XORI => self.execute_xori(instruction),
                InstructionType::ORI => self.execute_ori(instruction),
                InstructionType::ANDI => self.execute_andi(instruction),
                InstructionType::SLLI => self.execute_slli(instruction),
                InstructionType::SRLI => self.execute_srli(instruction),
                InstructionType::SRAI => self.execute_srai(instruction),
                InstructionType::ADD => self.execute_add(instruction),
                InstructionType::SUB => self.execute_sub(instruction),
                InstructionType::SLL => self.execute_sll(instruction),
                InstructionType::SLT => self.execute_slt(instruction),
                InstructionType::SLTU => self.execute_sltu(instruction),
                InstructionType::XOR => self.execute_xor(instruction),
                InstructionType::SRL => self.execute_srl(instruction),
                InstructionType::SRA => self.execute_sra(instruction),
                InstructionType::OR => self.execute_or(instruction),
                InstructionType::AND => self.execute_and(instruction),
                InstructionType::ECALL => self.pc += 4,
                InstructionType::CSRRW => self.pc += 4,
                InstructionType::CSRRS => self.pc += 4,
                InstructionType::CSRRC => self.pc += 4,
                InstructionType::CSRRWI => self.pc += 4,
                InstructionType::CSRRSI => self.pc += 4,
                InstructionType::CSRRCI => self.pc += 4,
                InstructionType::EBREAK => self.halted = true,
            }
            _ => ()
        }
    }

    pub fn execute_lui(&mut self, instruction: &Instruction) {
        let rd = instruction.get_rd();
        let imm = instruction.get_imm_u() << 12;

        self.registers.set(rd as usize, imm);

        self.pc += 4;
    }


    pub fn execute_auipc(&mut self, instruction: &Instruction) {
        let rd = instruction.get_rd();
        let imm = instruction.get_imm_u() << 12;

        self.registers.set(rd as usize, imm + self.pc as u32);

        self.pc += 4;
    }


    pub fn execute_jal(&mut self, instruction: &Instruction) {
        let rd = instruction.get_rd();
        let imm = instruction.get_imm_j();

        self.registers.set(rd as usize, (self.pc + 4) as u32);

        self.pc += imm as usize;
    }


    pub fn execute_jalr(&mut self, instruction: &Instruction) {
        let rd = instruction.get_rd();
        let rs = instruction.get_rs1();
        let imm = instruction.get_imm_i();

        let rs_value = self.registers.get(rs as usize);
        self.registers.set(rd as usize, (self.pc + 4) as u32);

        self.pc = ((imm + rs_value) & 0xFFFFFE) as usize;
    }


    pub fn execute_bne(&mut self, instruction: &Instruction) {
        let rs1 = instruction.get_rs1();
        let rs2 = instruction.get_rs2();
        let imm = instruction.get_imm_b();

        let rs1_value = self.registers.get(rs1 as usize);
        let rs2_value = self.registers.get(rs2 as usize);

        let pc_increment = if rs1_value != rs2_value { imm } else { 4 };
        self.pc += pc_increment as usize;
    }


    pub fn execute_blt(&mut self, instruction: &Instruction) {
        let rs1 = instruction.get_rs1();
        let rs2 = instruction.get_rs2();
        let imm = instruction.get_imm_b();

        let rs1_value = self.registers.get(rs1 as usize);
        let rs2_value = self.registers.get(rs2 as usize);

        let pc_increment = if (rs1_value as i32) < rs2_value as i32 { imm } else { 4 };
        self.pc += pc_increment as usize;
    }


    pub fn execute_bge(&mut self, instruction: &Instruction) {
        let rs1 = instruction.get_rs1();
        let rs2 = instruction.get_rs2();
        let imm = instruction.get_imm_b();

        let rs1_value = self.registers.get(rs1 as usize);
        let rs2_value = self.registers.get(rs2 as usize);

        let pc_increment = if rs1_value as i32 >= rs2_value as i32 { imm } else { 4 };
        self.pc += pc_increment as usize;
    }


    pub fn execute_bltu(&mut self, instruction: &Instruction) {
        let rs1 = instruction.get_rs1();
        let rs2 = instruction.get_rs2();
        let imm = instruction.get_imm_b();

        let rs1_value = self.registers.get(rs1 as usize);
        let rs2_value = self.registers.get(rs2 as usize);

        let pc_increment = if rs1_value < rs2_value { imm } else { 4 };
        self.pc += pc_increment as usize;
    }


    pub fn execute_bgeu(&mut self, instruction: &Instruction) {
        let rs1 = instruction.get_rs1();
        let rs2 = instruction.get_rs2();
        let imm = instruction.get_imm_b();

        let rs1_value = self.registers.get(rs1 as usize);
        let rs2_value = self.registers.get(rs2 as usize);

        let pc_increment = if rs1_value >= rs2_value { imm } else { 4 };
        self.pc += pc_increment as usize;
    }


    pub fn execute_beq(&mut self, instruction: &Instruction) {
        let rs1 = instruction.get_rs1();
        let rs2 = instruction.get_rs2();
        let imm = instruction.get_imm_b();

        let rs1_value = self.registers.get(rs1 as usize);
        let rs2_value = self.registers.get(rs2 as usize);

        let pc_increment = if rs1_value == rs2_value { imm } else { 4 };
        self.pc += pc_increment as usize;
    }

    pub fn execute_addi(&mut self, instruction: &Instruction) {
        let rd = instruction.get_rd();
        let rs1 = instruction.get_rs1();
        let imm = instruction.get_imm_i();

        let rs1_value = self.registers.get(rs1 as usize);
        self.registers.set(rd as usize, rs1_value + imm);

        self.pc += 4;
    }


    pub fn execute_lbu(&mut self, instruction: &Instruction) {
        let rd = instruction.get_rd();
        let rs1 = instruction.get_rs1();
        let imm = instruction.get_imm_i();

        let rs1_value = self.registers.get(rs1 as usize);
        let data = self.memory.get8((rs1_value + imm) as usize);
        self.registers.set(rd as usize, data as u32);
        self.pc += 4;
    }

    pub fn execute_lb(&mut self, instruction: &Instruction) {
        let rd = instruction.get_rd();
        let rs1 = instruction.get_rs1();
        let imm = instruction.get_imm_i();

        let rs1_value = self.registers.get(rs1 as usize);
        let data = self.memory.get8_sx((rs1_value + imm) as usize);
        self.registers.set(rd as usize, data as u32);
        self.pc += 4;
    }

    pub fn execute_lhu(&mut self, instruction: &Instruction) {
        let rd = instruction.get_rd();
        let rs1 = instruction.get_rs1();
        let imm = instruction.get_imm_i();

        let rs1_value = self.registers.get(rs1 as usize);
        let data = self.memory.get16((rs1_value + imm) as usize);
        self.registers.set(rd as usize, data as u32);
        self.pc += 4;
    }


    pub fn execute_lh(&mut self, instruction: &Instruction) {
        let rd = instruction.get_rd();
        let rs1 = instruction.get_rs1();
        let imm = instruction.get_imm_i();

        let rs1_value = self.registers.get(rs1 as usize);
        let data = self.memory.get16_sx((rs1_value + imm) as usize);
        self.registers.set(rd as usize, data);
        self.pc += 4;
    }


    pub fn execute_lw(&mut self, instruction: &Instruction) {
        let rd = instruction.get_rd();
        let rs1 = instruction.get_rs1();
        let imm = instruction.get_imm_i();

        let rs1_value = self.registers.get(rs1 as usize);
        let data = self.memory.get32((rs1_value + imm) as usize);
        self.registers.set(rd as usize, data);
        self.pc += 4;
    }


    pub fn execute_sb(&mut self, instruction: &Instruction) {
        let rs1 = instruction.get_rs1();
        let rs2 = instruction.get_rs2();
        let imm = instruction.get_imm_s();

        let rs1_value = self.registers.get(rs1 as usize);
        let rs2_value = self.registers.get(rs2 as usize);

        self.memory.set8(rs2_value as u8, (rs1_value + imm) as usize);

        self.pc += 4;
    }


    pub fn execute_sh(&mut self, instruction: &Instruction) {
        let rs1 = instruction.get_rs1();
        let rs2 = instruction.get_rs2();
        let imm = instruction.get_imm_s();

        let rs1_value = self.registers.get(rs1 as usize);
        let rs2_value = self.registers.get(rs2 as usize);

        self.memory.set16(rs2_value as u16, (rs1_value + imm) as usize);

        self.pc += 4;
    }


    pub fn execute_sw(&mut self, instruction: &Instruction) {
        let rs1 = instruction.get_rs1();
        let rs2 = instruction.get_rs2();
        let imm = instruction.get_imm_s();

        let rs1_value = self.registers.get(rs1 as usize);
        let rs2_value = self.registers.get(rs2 as usize);

        self.memory.set32(rs2_value, (rs1_value + imm) as usize);

        self.pc += 4;
    }


    pub fn execute_slti(&mut self, instruction: &Instruction) {
        let rs1 = instruction.get_rs1();
        let rs2 = instruction.get_rs2();
        let imm = instruction.get_imm_i();

        let rs1_value = self.registers.get(rs1 as usize);

        self.registers.set(rs2 as usize, if (rs1_value as i32) < imm as i32 { 1 } else { 0 });

        self.pc += 4;
    }


    pub fn execute_sltiu(&mut self, instruction: &Instruction) {
        let rs1 = instruction.get_rs1();
        let rs2 = instruction.get_rs2();
        let imm = instruction.get_imm_i();

        let rs1_value = self.registers.get(rs1 as usize);

        self.registers.set(rs2 as usize, if rs1_value < imm { 1 } else { 0 });

        self.pc += 4;
    }


    pub fn execute_xori(&mut self, instruction: &Instruction) {
        let rd = instruction.get_rd();
        let rs1 = instruction.get_rs1();
        let imm = instruction.get_imm_i();

        let rs1_value = self.registers.get(rs1 as usize);

        self.registers.set(rd as usize, rs1_value ^ imm);

        self.pc += 4;
    }


    pub fn execute_ori(&mut self, instruction: &Instruction) {
        let rd = instruction.get_rd();
        let rs1 = instruction.get_rs1();
        let imm = instruction.get_imm_i();

        let rs1_value = self.registers.get(rs1 as usize);

        self.registers.set(rd as usize, rs1_value | imm);

        self.pc += 4;
    }


    pub fn execute_andi(&mut self, instruction: &Instruction) {
        let rd = instruction.get_rd();
        let rs1 = instruction.get_rs1();
        let imm = instruction.get_imm_i();

        let rs1_value = self.registers.get(rs1 as usize);

        self.registers.set(rd as usize, rs1_value & imm);

        self.pc += 4;
    }


    pub fn execute_slli(&mut self, instruction: &Instruction) {
        let rd = instruction.get_rd();
        let rs1 = instruction.get_rs1();
        let imm = instruction.get_shamt();

        let rs1_value = self.registers.get(rs1 as usize);

        self.registers.set(rd as usize, rs1_value << imm);

        self.pc += 4;
    }


    pub fn execute_srli(&mut self, instruction: &Instruction) {
        let rd = instruction.get_rd();
        let rs1 = instruction.get_rs1();
        let imm = instruction.get_shamt();

        let rs1_value = self.registers.get(rs1 as usize);

        self.registers.set(rd as usize, rs1_value >> imm);

        self.pc += 4;
    }


    pub fn execute_srai(&mut self, instruction: &Instruction) {
        let rd = instruction.get_rd();
        let rs1 = instruction.get_rs1();
        let imm = instruction.get_shamt();

        let rs1_value = self.registers.get(rs1 as usize);

        self.registers.set(rd as usize, (rs1_value as i32 >> imm) as u32);

        self.pc += 4;
    }


    pub fn execute_add(&mut self, instruction: &Instruction) {
        let rd = instruction.get_rd();
        let rs1 = instruction.get_rs1();
        let rs2 = instruction.get_rs2();

        let rs1_value = self.registers.get(rs1 as usize);
        let rs2_value = self.registers.get(rs2 as usize);

        self.registers.set(rd as usize, (std::num::Wrapping(rs1_value) + std::num::Wrapping(rs2_value)).0);

        self.pc += 4;
    }

    pub fn execute_sub(&mut self, instruction: &Instruction) {
        let rd = instruction.get_rd();
        let rs1 = instruction.get_rs1();
        let rs2 = instruction.get_rs2();

        let rs1_value = self.registers.get(rs1 as usize);
        let rs2_value = self.registers.get(rs2 as usize);

        self.registers.set(rd as usize, (std::num::Wrapping(rs1_value) - std::num::Wrapping(rs2_value)).0);

        self.pc += 4;
    }

    pub fn execute_sll(&mut self, instruction: &Instruction) {
        let rd = instruction.get_rd();
        let rs1 = instruction.get_rs1();
        let rs2 = instruction.get_rs2();

        let rs1_value = self.registers.get(rs1 as usize);
        let rs2_value = self.registers.get(rs2 as usize);

        self.registers.set(rd as usize, rs1_value << (rs2_value & 0b11111));

        self.pc += 4;
    }


    pub fn execute_slt(&mut self, instruction: &Instruction) {
        let rd = instruction.get_rd();
        let rs1 = instruction.get_rs1();
        let rs2 = instruction.get_rs2();

        let rs1_value = self.registers.get(rs1 as usize);
        let rs2_value = self.registers.get(rs2 as usize);

        self.registers.set(rd as usize, if (rs1_value as i32) < rs2_value as i32 { 1 } else { 0 });

        self.pc += 4;
    }


    pub fn execute_sltu(&mut self, instruction: &Instruction) {
        let rd = instruction.get_rd();
        let rs1 = instruction.get_rs1();
        let rs2 = instruction.get_rs2();

        let rs1_value = self.registers.get(rs1 as usize);
        let rs2_value = self.registers.get(rs2 as usize);

        self.registers.set(rd as usize, if rs1_value < rs2_value { 1 } else { 0 });

        self.pc += 4;
    }


    pub fn execute_xor(&mut self, instruction: &Instruction) {
        let rd = instruction.get_rd();
        let rs1 = instruction.get_rs1();
        let rs2 = instruction.get_rs2();

        let rs1_value = self.registers.get(rs1 as usize);
        let rs2_value = self.registers.get(rs2 as usize);

        self.registers.set(rd as usize, rs1_value ^ rs2_value);

        self.pc += 4;
    }


    pub fn execute_srl(&mut self, instruction: &Instruction) {
        let rd = instruction.get_rd();
        let rs1 = instruction.get_rs1();
        let rs2 = instruction.get_rs2();

        let rs1_value = self.registers.get(rs1 as usize);
        let rs2_value = self.registers.get(rs2 as usize);

        self.registers.set(rd as usize, rs1_value >> (rs2_value & 0b11111));

        self.pc += 4;
    }


    pub fn execute_sra(&mut self, instruction: &Instruction) {
        let rd = instruction.get_rd();
        let rs1 = instruction.get_rs1();
        let rs2 = instruction.get_rs2();

        let rs1_value = self.registers.get(rs1 as usize);
        let rs2_value = self.registers.get(rs2 as usize);

        self.registers.set(rd as usize, (rs1_value as i32 >> (rs2_value & 0b11111)) as u32);

        self.pc += 4;
    }


    pub fn execute_or(&mut self, instruction: &Instruction) {
        let rd = instruction.get_rd();
        let rs1 = instruction.get_rs1();
        let rs2 = instruction.get_rs2();

        let rs1_value = self.registers.get(rs1 as usize);
        let rs2_value = self.registers.get(rs2 as usize);

        self.registers.set(rd as usize, rs1_value | rs2_value);

        self.pc += 4;
    }


    pub fn execute_and(&mut self, instruction: &Instruction) {
        let rd = instruction.get_rd();
        let rs1 = instruction.get_rs1();
        let rs2 = instruction.get_rs2();

        let rs1_value = self.registers.get(rs1 as usize);
        let rs2_value = self.registers.get(rs2 as usize);

        self.registers.set(rd as usize, rs1_value & rs2_value);

        self.pc += 4;
    }
}