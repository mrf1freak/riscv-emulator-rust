use std::{fs, io};

#[derive(Clone)]
pub struct Memory {
    memory: Vec<u8>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        return Self { memory: vec![0; size] };
    }

    pub fn load_file(&mut self, file: &str) -> Result<(), io::Error> {
        let memory = fs::read(file)?;
        for (index, byte) in memory.iter().enumerate() {
           self.set8(*byte, index);
        }
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.memory.len()
    }

    pub fn get8(&self, index: usize) -> u8 {
        self.memory[index]
    }

    pub fn get16(&self, index: usize) -> u16 {
        let last = self.get8(index) as u16;
        let first = self.get8(index + 1) as u16;
        return first << 8 | last;
    }

    pub fn get32(&self, index: usize) -> u32 {
        let last = self.get16(index) as u32;
        let first = self.get16(index + 2) as u32;
        return first << 16 | last;
    }

    pub fn get8_sx(&self, index: usize) -> u32 {
        let data = self.get8(index);
        if data & 0x80 == 0 {
            data as u32
        } else {
            data as u32 | 0xFFFFFF00
        }
    }

    pub fn get16_sx(&self, index: usize) -> u32 {
        let data = self.get16(index);
        if data & 0x8000 == 0 {
            data as u32
        } else {
            data as u32 | 0xFFFF0000
        }
    }

    pub fn set8(&mut self, data: u8, index: usize) {
        self.memory[index] = data;
    }

    pub fn set16(&mut self, data: u16, index: usize) {
        self.set8(data as u8, index);
        self.set8((data >> 8) as u8, index + 1);
    }

    pub fn set32(&mut self, data: u32, index: usize) {
        self.set16(data as u16, index);
        self.set16((data >> 16) as u16, index + 2);
    }

    pub fn dump(&self) {
        for i in 1..self.memory.len() / 16 {
            print!("{:08x}  ", i * 16);

            for j in 0..16 {
                print!("{:02x} ", self.get8(i * 16 + j));
                if j == 7 { print!(" ") }
            }

            print!("  *");
            for j in 0..16 {
                let byte = self.get8(i * 16 + j);
                match byte {
                    0x20..=0x7E =>
                        print!("{}", byte as char),
                    _ => print!("."),
                }
            }
            print!("*");

            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::memory::Memory;

    #[test]
    fn test_get32() {
        let mut memory = Memory::new(8);
        memory.set8(0x00, 0);
        memory.set8(0x11, 1);
        memory.set8(0x22, 2);
        memory.set8(0x33, 3);
        memory.set8(0x44, 4);
        memory.set8(0x55, 5);
        memory.set8(0x66, 6);
        memory.set8(0x77, 7);


        assert_eq!(memory.get16(0), 0x1100);
        assert_eq!(memory.get16(0), 0x2211);
        assert_eq!(memory.get16(2), 0x3322);

        assert_eq!(memory.get32(0), 0x33221100);
        assert_eq!(memory.get32(1), 0x44332211);
        assert_eq!(memory.get32(2), 0x55443322);
        assert_eq!(memory.get32(3), 0x66554433);
        assert_eq!(memory.get32(4), 0x77665544);
    }
}