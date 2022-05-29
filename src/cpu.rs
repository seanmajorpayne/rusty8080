const NUM_OF_INSTRUCTIONS: usize = 3;
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

#[derive(Copy, Clone)]
pub struct Cpu {
    a: u16,                          // Registers
    b: u16,                          //
    c: u16,                          //
    d: u16,                          //
    e: u16,                          //
    h: u16,                          //
    l: u16,                          //
    sp: u16,                        // Stack Pointer
    pc: usize,                      // Program Counter
    memory: [u16; MEMORY_SIZE_KB],
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
                i { name: "NOP", operation: Cpu::nop, cycles: 1 }, i { name: "LXI B,D16", operation: Cpu::lxi_b_d16, cycles: 3 }, i { name: "STAX B", operation: Cpu::stax_b, cycles: 2 },
            ],
            int_enable: 0,
        }
    }

    pub fn run(&mut self) {
        self.process_instruction();
    }

    // Instructions

    // 0x00	NOP	1
    // TODO: Handle illegal opcodes
    fn nop(&mut self) {
        return
    }

    // 0x01	LXI B,D16	3		B <- byte 3, C <- byte 2
    fn lxi_b_d16(&mut self) {
        self.c = self.memory[self.pc + 1];
        self.b = self.memory[self.pc + 2];
        self.pc += 2;
    }

    // 0x02	STAX B	1		(BC) <- A
    fn stax_b(&mut self) {
        let addr: usize = (self.b << 8 | self.c) as usize;
        self.memory[addr] = self.a;
        return
    }

    fn process_instruction(&mut self) {
        let opcode = self.memory[self.pc] as usize;

        let instruction = self.instructions[opcode];

        println!("{}", instruction.name);
        instruction.call(self);
        self.pc += 1;
    }
}

mod tests {
    use super::Cpu;

    const DATA_ONE: u16 = 1;
    const DATA_TWO: u16 = 2;

    #[test]
    fn nop_increments_program_counter() {
        let mut cpu = Cpu::new();
        let cpu_before = cpu.clone();

        cpu.process_instruction();

        assert_eq!(cpu_before.flags, cpu.flags);
        assert_eq!(cpu_before.memory, cpu.memory);
        assert_eq!(cpu_before.pc, cpu.pc - 1);
    }

    #[test]
    fn lxi_b_d16_changes_c_and_b() {
        let mut cpu = Cpu::new();
        cpu.memory[0] = 0x01;
        cpu.memory[1] = DATA_ONE;
        cpu.memory[2] = DATA_TWO;
        let cpu_before = cpu.clone();

        cpu.process_instruction();

        assert_eq!(cpu_before.flags, cpu.flags);
        assert_eq!(cpu_before.memory, cpu.memory);
        assert_eq!(cpu_before.memory[1], cpu.c);
        assert_eq!(cpu_before.pc, cpu.pc - 3);
    }

    #[test]
    fn stax_b_moves_a_into_addr_bc() {
        let mut cpu = Cpu::new();
        cpu.memory[0] = 0x02;
        cpu.b = 1;
        cpu.c = 2;
        let cpu_before = cpu.clone();

        cpu.process_instruction();

        assert_eq!(cpu_before.flags, cpu.flags);
        assert_eq!(cpu.memory[258], cpu_before.a);
    }
}

