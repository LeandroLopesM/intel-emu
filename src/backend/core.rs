use std::collections::HashMap;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Intel8080 {
    memory: Vec<u8>,
    pub registers: RegisterList,
}

impl Intel8080 {
    pub fn new(memsize: Option<usize>) -> Self {
        let mut mem = Vec::new();

        if let Some(size) = memsize {
            mem.reserve(size);
        }

        return Self {
            memory: mem,
            registers: RegisterList::from_vec(vec![
                ("PC", Register::DWord(0)),
                ("SP", Register::DWord(0)),
                ("PSW", Register::DWord(0)),
                    ("A", Register::HiOf("PSW")),
                    ("S", Register::BitOf("PSW", 0)), // signed
                    ("Z", Register::BitOf("PSW", 1)), // Zero
                    ("I", Register::BitOf("PSW", 2)), // interrupt enable
                    ("P", Register::BitOf("PSW", 3)), // parity
                    ("CY", Register::BitOf("PSW", 4)), // Carry
                    ("AC", Register::BitOf("PSW", 5)), // AuxCy
                ("DE", Register::DWord(0)),
                    ("D", Register::HiOf("DE")),
                    ("E", Register::LowOf("DE")),
                ("BC", Register::DWord(0)),
                    ("B", Register::HiOf("BC")),
                    ("C", Register::LowOf("BC")),
                ("HL", Register::DWord(0)),
                    ("H", Register::HiOf("HL")),
                    ("L", Register::LowOf("HL")),
            ])
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Register {
    BitOf(&'static str, u8),
    HiOf(&'static str),
    LowOf(&'static str),
    Byte(u8),
    DWord(u16),
}

#[derive(Clone, Debug)]
pub struct RegisterList {
    list: HashMap<&'static str, Register>,
}

impl RegisterList {
    pub fn from_vec(vec: Vec<(&'static str, Register)>) -> Self {
        Self {
            list: vec.iter().cloned().collect()
        }
    }

    pub fn get_dword<S: AsRef<str>>(&self, name: S) -> Result<u16, &'static str> {
        let reg = self.list.get(name.as_ref());

        if reg.is_none() {
            return Err("No such register");
        }

        match reg.unwrap() {
            Register::DWord(val) => Ok(*val),
            _ => Err("Register type doesn't support DWord")
        }
    }
    
    pub fn set_dword<S: AsRef<str>>(&mut self, name: S, val: u16) {
        let reg = self.list.get_mut(name.as_ref());

        if let Register::DWord(dw) = reg.unwrap() {
            *dw = val;
        }
    }
}