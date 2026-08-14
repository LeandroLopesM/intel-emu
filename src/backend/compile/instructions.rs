#![allow(dead_code)]
// #[dynamic]
// static INSTRUCTION_MAP: HashMap<&'static str, Instruction> = HashMap::from([
//     ("MOV", Instruction::new("MOV", vec![( "Destination", Arg::BRegister ), ( "Source", Arg::BRegister )], "Move register to register")),
//     ("MVI", Instruction::new("MVI", vec![( "Destination", Arg::BRegister ), ( "Source", Arg::BRegister )], "Move immediate to register")),

//     ("LXI", Instruction::new("LXI", vec![( "RegisterPair", Arg::PairRegister ), ( "Address", Arg::DWord )], "Load register pair immediate")),

//     ("LDA", Instruction::new("LDA", vec![( "Address", Arg::DWord )], "Load A from memory")),
//     ("STA", Instruction::new("STA", vec![( "Address", Arg::DWord )], "Store A to memory")),

//     ("LHLD", Instruction::new("LHLD", vec![( "Address", Arg::DWord )], "Load HL from memory")),
//     ("SHLD", Instruction::new("SHLD", vec![( "Address", Arg::DWord )], "Store HL to memory")),

//     ("LDAX", Instruction::new("LDAX", vec![( "BC or DE", Arg::Register )], "Load indirect through BC or DE")),
//     ("STAX", Instruction::new("STAX", vec![( "BC or DE", Arg::Register )], "Store indirect through BC or DE")),

//     ("XCHG", Instruction::new("XCHG", vec![], "Exchange the values in DE and HL")),

//     ("ADD", Instruction::new("ADD", vec![( "Source", Arg::Register )], "Add register to A")),
//     ("ADC", Instruction::new("ADC", vec![( "Source", Arg::Register )], "Add register to A w/ carry")),
//     ("ADI", Instruction::new("ADI", vec![( "Value", Arg::Byte )], "Add byte to A")),
//     ("ACI", Instruction::new("ACI", vec![( "Value", Arg::Byte )], "Add byte to A w/ carry")),

//     ("SUB", Instruction::new("SUB", vec![( "Source", Arg::Register )], "Subtract register from A")),
//     ("SBB", Instruction::new("SBB", vec![( "Source", Arg::Register )], "Subtract register from A w/ borrow")),
//     ("SUI", Instruction::new("SUI", vec![( "Value", Arg::Byte )], "Subtract byte from A")),
//     ("SBI", Instruction::new("SBI", vec![( "Value", Arg::Byte )], "Subtract byte from A w/ borrow")),

//     ("INR", Instruction::new("INR", vec![( "Source", Arg::Register )], "Increment register")),
//     ("DCR", Instruction::new("DCR", vec![( "Source", Arg::Register )], "Decrement register")),

//     ("INX", Instruction::new("INX", vec![( "Source", Arg::RegisterPair )], "Increment register pair")),
//     ("DCX", Instruction::new("DCX", vec![( "Source", Arg::RegisterPair )], "Decrement register pair")),

//     ("DAD", Instruction::new("DAD", vec![( "Source", Arg::RegisterPair )], "Add register pair to HL")),
//     ("DAA", Instruction::new("DAA", vec![], "Decimal adjust accumulator")),

//     ("ANA", Instruction::new("ANA", vec![( "Source", Arg::Register )], "AND register with A")),
//         ("ANI", Instruction::new("ANI", vec![( "Value", Arg::Byte )], "AND byte with A")),
//     ("ORA", Instruction::new("ORA", vec![( "Source", Arg::Register )], "OR register with A")),
//         ("ORI", Instruction::new("ORI", vec![( "Value", Arg::Byte )], "OR byte with A")),
//     ("XRA", Instruction::new("XRA", vec![( "Source", Arg::Register )], "XOR register with A")),
//         ("XRI", Instruction::new("XRI", vec![( "Value", Arg::Byte )], "XOR byte with A")),

//     ("CMP", Instruction::new("CMP", vec![( "Source", Arg::Register )], "Compare register with A")),
//         ("CPI", Instruction::new("CPI", vec![( "Source", Arg::Register )], "Compare byte with A")),

//     ("RLC", Instruction::new("RLC", vec![], "Rotate A left")),
//         ("RAL", Instruction::new("RAL", vec![], "Rotate A left through carry")),
//     ("RLC", Instruction::new("RLC", vec![], "Rotate A right")),
//         ("RAR", Instruction::new("RAR", vec![], "Rotate A right through carry")),
//     ("CMA", Instruction::new("RAR", vec![], "Rotate A right through carry")),
//         ("CMC", Instruction::new("RAR", vec![], "Rotate A right through carry")),
//     ("STC", Instruction::new("RAR", vec![], "Set Carry flag")),
//     ("JMP",  vec![("Address", Arg::DWord)], "Unconditional jump"),
//         ("Jccc", vec![("Address", Arg::DWord)], "Conditional jump"),
//     ("CALL", vec![("Address", Arg::DWord)], "Unconditional subroutine call"),
//         ("Cccc", vec![("Address", Arg::DWord)], "Conditional subroutine call"),
//     ("RET",  vec![], "Unconditional return from subroutine"),
//         ("Rccc", vec![  ], "Conditional return from subroutine"),
//     ("RST", vec![  ], "Restart (Call n*8)"),
//     ("PCHL", vec![  ], "Jump to address in H:L"),
//     ("PUSH", vec![("RegisterPair", Arg::RegisterPair)], "Push register pair on the stack"),
//     ("POP", vec![("RegisterPair", Arg::RegisterPair)], "Pop  register pair from the stack"),
//     ("XTHL", vec![  ], "Swap H:L with top word on stack"),
//     ("SPHL", vec![  ], "Set SP to content of H:L"),
//     ("IN", vec![("Port", Arg::Byte)], "Read input port into A"),
//     ("OUT", vec![("Port", Arg::Byte)], "Write A to output port"),
//     ("EI", vec![  ], "Enable interrupts"),
//         ("DI", vec![  ], "Disable interrupts"),
//     ("HLT", vec![  ], "Halt processor"),
//     ("NOP", vec![  ], "No operation"),
// ]);

#[derive(Clone, Debug)]
pub struct Instruction {
    args: Vec<(&'static str, Arg)>, // (argname argval)
    name: &'static str,
}

impl Instruction {
    pub fn new(name: &'static str, args: Vec<(&'static str, Arg)>) -> Self {
        Self { args, name }
    }
}

#[derive(Clone, Debug)]
pub enum Arg {
    DWord(u16),
    Byte(u8),
    BRegister,
}

enum Register {
    PC,
    SP,
    PSW,
    A,
    S,
    Z,
    I,
    P,
    CY,
    AC,

    DE,
    D,
    E,
    BC,
    B,
    C,
    HL,
    H,
    L,
}
