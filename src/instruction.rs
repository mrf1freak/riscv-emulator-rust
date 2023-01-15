use std::fmt::{Display, Formatter};

pub struct Instruction {
    instruction: u32,
}

#[derive(Debug)]
pub enum InstructionType {
    LUI,
    AUIPC,
    JAL,
    JALR,
    BEQ,
    BNE,
    BLT,
    BGE,
    BLTU,
    BGEU,
    LB,
    LH,
    LW,
    LBU,
    LHU,
    SB,
    SH,
    SW,
    ADDI,
    SLTI,
    SLTIU,
    XORI,
    ORI,
    ANDI,
    SLLI,
    SRLI,
    SRAI,
    ADD,
    SUB,
    SLL,
    SLT,
    SLTU,
    XOR,
    SRL,
    SRA,
    OR,
    AND,
    ECALL,
    EBREAK,
    CSRRW,
    CSRRS,
    CSRRC,
    CSRRWI,
    CSRRSI,
    CSRRCI,

}

impl Instruction {
    pub fn from_u32(instruction: u32) -> Self {
        Self { instruction }
    }

    pub fn opcode(&self) -> u8 {
        (self.instruction & 0x7F) as u8
    }

    pub fn _type(&self) -> Result<InstructionType, String> {
        let error: Result<InstructionType, String> = Err(format!("Illegal Instruction {:#x}", self.instruction));

        return match self.opcode() {
            0b0110111 => Ok(InstructionType::LUI),
            0b0010111 => Ok(InstructionType::AUIPC),
            0b1101111 => Ok(InstructionType::JAL),
            0b1100111 => Ok(InstructionType::JALR),

            0b1100011 => match self.get_funct3() {
                0b000 => Ok(InstructionType::BEQ),
                0b001 => Ok(InstructionType::BNE),
                0b100 => Ok(InstructionType::BLT),
                0b101 => Ok(InstructionType::BGE),
                0b110 => Ok(InstructionType::BLTU),
                0b111 => Ok(InstructionType::BGEU),
                _ => error
            }

            0b0000011 => match self.get_funct3() {
                0b000 => Ok(InstructionType::LB),
                0b001 => Ok(InstructionType::LH),
                0b010 => Ok(InstructionType::LW),
                0b100 => Ok(InstructionType::LBU),
                0b101 => Ok(InstructionType::LHU),
                _ => error
            }

            0b0100011 => match self.get_funct3() {
                0b000 => Ok(InstructionType::SB),
                0b001 => Ok(InstructionType::SH),
                0b010 => Ok(InstructionType::SW),
                _ => error
            }

            0b0010011 => match self.get_funct3() {
                0b000 => Ok(InstructionType::ADDI),
                0b010 => Ok(InstructionType::SLTI),
                0b011 => Ok(InstructionType::SLTIU),
                0b100 => Ok(InstructionType::XORI),
                0b110 => Ok(InstructionType::ORI),
                0b111 => Ok(InstructionType::ANDI),
                0b001 => Ok(InstructionType::SLLI),
                0b101 => match self.get_funct7() {
                    0b0000000 => Ok(InstructionType::SRLI),
                    0b0100000 => Ok(InstructionType::SRAI),
                    _ => error
                }
                _ => error
            }

            0b0110011 => match self.get_funct3() {
                0b000 => match self.get_funct7() {
                    0b0000000 => Ok(InstructionType::ADD),
                    0b0100000 => Ok(InstructionType::SUB),
                    _ => error
                }

                0b001 => Ok(InstructionType::SLL),
                0b010 => Ok(InstructionType::SLT),
                0b011 => Ok(InstructionType::SLTU),
                0b100 => Ok(InstructionType::XOR),
                0b101 => match self.get_funct7() {
                    0b0000000 => Ok(InstructionType::SRL),
                    0b0100000 => Ok(InstructionType::SRA),
                    _ => error
                }
                0b110 => Ok(InstructionType::OR),
                0b111 => Ok(InstructionType::AND),
                _ => error
            }

            0b1110011 => match self.get_funct3() {
                0b000 => match self.get_imm_i() {
                    0 => Ok(InstructionType::ECALL),
                    1 => Ok(InstructionType::EBREAK),
                    _ => error
                }
                0b001 => Ok(InstructionType::CSRRW),
                0b010 => Ok(InstructionType::CSRRS),
                0b011 => Ok(InstructionType::CSRRC),
                0b101 => Ok(InstructionType::CSRRWI),
                0b110 => Ok(InstructionType::CSRRSI),
                0b111 => Ok(InstructionType::CSRRCI),
                _ => error
            }

            _ => error
        };
    }

    pub fn get_mnemonic(&self) -> Option<String> {
        match self._type() {
            Ok(t) => Some(format!("{:?}", t).to_lowercase()),
            _ => None
        }
    }

    pub fn get_rd(&self) -> u8 {
        return (self.instruction << 20 >> 27) as u8;
    }

    pub fn get_funct3(&self) -> u8 {
        return (self.instruction << 17 >> 29) as u8;
    }

    pub fn get_rs1(&self) -> u8 {
        return (self.instruction << 12 >> 27) as u8;
    }

    pub fn get_rs2(&self) -> u8 {
        return (self.instruction << 7 >> 27) as u8;
    }

    pub fn get_funct7(&self) -> u8 {
        return (self.instruction >> 25) as u8;
    }

    pub fn get_imm_i(&self) -> u32 {
        let mut insn = self.instruction;
        insn >>= 20;
        if insn & 0x800 != 0 { insn |= 0xFFFFF000; }
        return insn;
    }

    pub fn get_imm_u(&self) -> u32 {
        return self.instruction >> 12;
    }

    pub fn get_imm_b(&self) -> u32 {
        let rightmost = self.instruction >> 8 << 1 & 0b11110;
        let right = self.instruction << 1 >> 21 & 0b11111100000;
        let single_bit = self.instruction << 4 & (0b1 << 11);
        let left_bit = (self.instruction & (0b1 << 31)) >> 19;
        let complete = rightmost | right | single_bit | left_bit;

        if complete >> 12 != 0 {
            return complete | 0xFFFFF000;
        }

        return complete;
    }

    pub fn get_imm_s(&self) -> u32 {
        let right = self.instruction >> 7 & (0xFFFFFFFF >> 27);
        let left = self.instruction >> 25;
        let complete = (left << 5) | right;

        if 0x800 & complete != 0 {
            return complete | 0xFFFFF000;
        }

        return complete;
    }

    pub fn get_imm_j(&self) -> u32 {
        let right = self.instruction >> 21 << 1;
        let center = self.instruction & 0x000FF000;
        let complete = center | right;
        if complete & 0x00080000 != 0 {
            return complete | 0xFFF00000;
        }

        let eleventh_bit = (self.instruction >> 20) & 0x1;
        return complete | eleventh_bit << 11;
    }

    pub fn get_shamt(&self) -> u8 {
        (self.instruction >> 20 & 0b11111) as u8
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self._type() {
            Ok(_type) => {
                write!(f, "{:5} ", self.get_mnemonic().unwrap_or("".to_string()))?;
                match _type {
                    InstructionType::LUI => write!(f, "x{},{:#x}", self.get_rd(), self.get_imm_u()),
                    InstructionType::AUIPC => write!(f, "x{},{:#x}", self.get_rd(), self.get_imm_u()),

                    InstructionType::JAL => write!(f, "x{},{:#x}", self.get_rd(), self.get_imm_j()),
                    InstructionType::JALR => write!(f, "x{},{}(x{})", self.get_rd(), self.get_imm_i(), self.get_rs1()),

                    InstructionType::BEQ |
                    InstructionType::BNE |
                    InstructionType::BLT |
                    InstructionType::BGE |
                    InstructionType::BLTU |
                    InstructionType::BGEU
                    => write!(f, "x{},x{},{:#x}", self.get_rs1(), self.get_rs2(), self.get_imm_b()),

                    InstructionType::LB |
                    InstructionType::LH |
                    InstructionType::LW |
                    InstructionType::LBU |
                    InstructionType::LHU
                    => write!(f, "x{},{:#x},x{}", self.get_rd(), self.get_imm_i(), self.get_rs1()),

                    InstructionType::SB |
                    InstructionType::SH |
                    InstructionType::SW
                    => write!(f, "x{},{:#x}(x{})", self.get_rs2(), self.get_imm_s(), self.get_rs1()),

                    InstructionType::ADDI |
                    InstructionType::SLTI |
                    InstructionType::SLTIU |
                    InstructionType::XORI |
                    InstructionType::ORI |
                    InstructionType::ANDI
                    => write!(f, "x{},x{},{:#x}", self.get_rd(), self.get_rs1(), self.get_imm_i()),

                    InstructionType::SLLI |
                    InstructionType::SRLI |
                    InstructionType::SRAI
                    => write!(f, "x{},x{},{:#x}", self.get_rd(), self.get_rs1(), self.get_shamt()),

                    InstructionType::ADD |
                    InstructionType::SUB |
                    InstructionType::SLL |
                    InstructionType::SLT |
                    InstructionType::SLTU |
                    InstructionType::XOR |
                    InstructionType::SRL |
                    InstructionType::SRA |
                    InstructionType::OR |
                    InstructionType::AND
                    => write!(f, "x{},x{},x{}", self.get_rd(), self.get_rs1(), self.get_rs2()),

                    InstructionType::ECALL |
                    InstructionType::EBREAK
                    => write!(f, ""),

                    InstructionType::CSRRW |
                    InstructionType::CSRRS |
                    InstructionType::CSRRC |
                    InstructionType::CSRRWI |
                    InstructionType::CSRRSI |
                    InstructionType::CSRRCI
                    => write!(f, "x{},{:#x},x{}", self.get_rd(), self.get_imm_i(), self.get_rs1()),
                }
            }
            Err(e) => write!(f, "{}", e)
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::instruction::Instruction;

    #[test]
    fn test_imm_u() {
        assert_eq!(Instruction::from_u32(0x00040137).get_imm_u(), 0x00040);
        assert_eq!(Instruction::from_u32(0x00008fb7).get_imm_u(), 0x00008);

        assert_eq!(Instruction::from_u32(0x00040137).get_imm_u(), 0x00040);
        assert_eq!(Instruction::from_u32(0x00001117).get_imm_u(), 0x1);
        assert_eq!(Instruction::from_u32(0x00000117).get_imm_u(), 0x0);
    }

    #[test]
    fn test_imm_i() {
        assert_eq!(Instruction::from_u32(0x000002ef).get_imm_i(), 0x0);
        assert_eq!(Instruction::from_u32(0x008002ef).get_imm_i(), 0x08);
    }
}