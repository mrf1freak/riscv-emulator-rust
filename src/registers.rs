pub struct Registers {
    registers: Vec<u32>,
}

impl Registers {
    pub fn new() -> Self {
        Self { registers: vec![0xF0F0F0F0; 32] }
    }

    pub fn set(&mut self, register: usize, data: u32) {
        if register > 0 { self.registers[register] = data }
    }

    pub fn get(&self, register: usize) -> u32 {
        if register == 0 { return 0; }
        self.registers[register]
    }

    pub fn dump(&self) {
        for (i, data) in self.registers.iter().enumerate() {
            if i % 8 == 0 { print!("x{:02}  ", i) }
            print!("{:08x} ", data);
            if i % 8 == 3 { print!(" ") }
            if i % 8 == 7 { println!() }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::registers::Registers;

    #[test]
    fn zero_register_always_zero() {
        let mut registers = Registers::new();

        assert_eq!(registers.get(0), 0);
        registers.set(0, 0xAA);
        assert_eq!(registers.get(0), 0);
    }


    #[test]
    fn all_zero_initialized() {
        let mut registers = Registers::new();

        for i in 1..24 {
            assert_eq!(registers.get(i), 0)
        }
    }

    #[test]
    fn registers_set_correctly() {
        let mut registers = Registers::new();

        registers.set(1, 0xAA);
        registers.set(2, 0xBBBB);
        registers.set(3, 0xCCCCCC);
        registers.set(4, 0xDDDDDDDD);

        assert_eq!(registers.get(1), 0xAA);
        assert_eq!(registers.get(2), 0xBBBB);
        assert_eq!(registers.get(3), 0xCCCCCC);
        assert_eq!(registers.get(4), 0xDDDDDDDD);
    }
}