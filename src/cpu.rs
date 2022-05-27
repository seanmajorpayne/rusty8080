const NUM_OF_INSTRUCTIONS: usize = 2;
const MEMORY_SIZE_KB: usize = 16000;

#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
#[derive(Debug)]
struct Flags {
    z: u8,
    s: u8,
    p: u8,
    cy: u8,
    ac: u8,
    pad: u8,
}

#[derive(Copy, Clone)]
struct Instruction {
    name: &'static str,
    operation: fn(&mut Cpu),
    cycles: u8,
}

impl Instruction {
    fn call(&self, cpu: &mut Cpu) {
        (self.operation)(cpu);
    }
}

pub struct Cpu {
    a: u8,                          // Registers
    b: u8,                          //
    c: u8,                          //
    d: u8,                          //
    e: u8,                          //
    h: u8,                          //
    l: u8,                          //
    sp: u16,                        // Stack Pointer
    pc: usize,                      // Program Counter
    memory: [u8; MEMORY_SIZE_KB],
    instructions: [Instruction; NUM_OF_INSTRUCTIONS],
    flags: Flags,
    int_enable: u8,
}

impl Cpu {
    pub fn new() -> Cpu {
        use Instruction as i;
        Cpu {
            a: 1, b: 1, c: 1,
            d: 1, e: 1, h: 1,
            l: 1, sp: 1, pc: 0,
            memory: [0; MEMORY_SIZE_KB],
            flags: Flags { z: 1, s: 1, p: 1, cy: 1, ac: 1, pad: 3 },
            instructions: [
                i { name: "NOP", operation: Cpu::nop, cycles: 1 }, i { name: "LXI B,D16", operation: Cpu::lxi_b_d16, cycles: 3 },
            ],
            int_enable: 0,
        }
    }

    pub fn run(&mut self) {
        self.process_instruction();
    }

    // Address Modes

    // imp - Address Mode: Implied
    // No additional data needed for the instruction
    fn imp(&self) {
        return
    }

    // Instructions

    // NOP Instruction
    // TODO: Handle illegal opcodes
    fn nop(&mut self) {
        println!("NOP");
        self.c = 14;
    }

    // LXI B,D16
    fn lxi_b_d16(&mut self) {
        self.c = self.memory[self.pc + 1];
        self.b = self.memory[self.pc + 2];
        self.pc += 2;
    }

    fn process_instruction(&mut self) {
        let opcode = self.memory[self.pc] as usize;

        let instruction = self.instructions[opcode];

        println!("{}", instruction.name);
        instruction.call(self);
    }
}

/*mod tests {
    use super::Cpu;

    #[test]
    fn opcode_00_increments_program_counter() {
        let mut cpu = Cpu::new();
        let cpu_before = cpu.clone();

        cpu.process_instruction();

        assert_eq!(cpu_before.flags, cpu.flags);
        assert_eq!(cpu_before.memory, cpu.memory);
        assert_eq!(cpu_before.pc, cpu.pc - 1)
    }
}
*/
