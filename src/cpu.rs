const NUM_OF_INSTRUCTIONS: usize = 10;
const MEMORY_SIZE_KB: usize = 16000;

#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
#[derive(Debug)]
struct Flags {
    z: bool,
    s: bool,
    p: bool,
    cy: bool,
    ac: bool,
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

// Return true if even parity
fn parity(num: u16, size: u8) -> bool {
    let mut count: u8 = 0;

    for _ in 0..size {
        if (num & 0x01) == 0x01 { count = count + 1 }
        num >> 1;
    }

    0 == (count & 0x01)
}

fn unimplemented_instruction() {
    println!("Implement me");
}

impl Cpu {
    pub fn new() -> Cpu {
        use Instruction as i;
        Cpu {
            a: 1, b: 1, c: 1,
            d: 1, e: 1, h: 1,
            l: 1, sp: 1, pc: 0,
            memory: [0; MEMORY_SIZE_KB],
            flags: Flags { z: true, s: true, p: true, cy: true, ac: true, pad: 3 },
            instructions: [
                i { name: "NOP", operation: Cpu::nop, cycles: 1 }, i { name: "LXI B,D16", operation: Cpu::lxi_b_d16, cycles: 3 }, i { name: "STAX B", operation: Cpu::stax_b, cycles: 2 }, i { name: "INX B", operation: Cpu::inx_b, cycles: 1 }, i { name: "INR B", operation: Cpu::inr_b, cycles: 1 }, i { name: "DCR B", operation: Cpu::dcr_b, cycles: 1 }, i { name: "MVI B,D8", operation: Cpu::mvi_b_d8, cycles: 2 }, i { name: "RLC 1", operation: Cpu::rlc_1, cycles: 1 }, i { name: "NOP", operation: Cpu::nop, cycles: 1 }, i { name: "DAD B", operation: Cpu::dad_b, cycles: 3 },
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

    // 0x03	INX B	1		BC <- BC+1
    fn inx_b(&mut self) {
        unimplemented_instruction();
    }

    // 0x04	INR B	1	Z, S, P, AC	B <- B+1
    fn inr_b(&mut self) {
        unimplemented_instruction();
    }

    // 0x05	DCR B	1	Z, S, P, AC	B <- B-1
    fn dcr_b(&mut self) {
        self.b -= 1;
        self.flags.z = (0 == self.b);
        self.flags.s = (self.b & 0x80 == 0x80);
        self.flags.p = parity(self.b, 8);
    }

    // 0x06	MVI B, D8	2		B <- byte 2
    fn mvi_b_d8(&mut self) {
        self.b = self.memory[self.pc + 1];
        self.pc += 1;
    }

    // 0x07	RLC	1	CY	A = A << 1; bit 0 = prev bit 7; CY = prev bit 7
    fn rlc_1(&mut self) {
        return
    }

    // 0x09	DAD B	1	CY	HL = HL + BC
    fn dad_b(&mut self) {
        let res: u16 = (self.h << 8 | self.l) + (self.b << 8 | self.c);
        self.h = (res & 0xff00) >> 8;
        self.l = (res & 0x00ff);
        self.flags.cy = (res as u32 & 0xffff0000) > 0;
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

    #[test]
    fn inx_b() {
        // TODO
        assert_eq!(1, 1);
    }

    #[test]
    fn inr_b() {
        // TODO
        assert_eq!(1, 1);
    }

    #[test]
    fn dcr_b_handles_zero_result() {
        let mut cpu = Cpu::new();
        cpu.memory[0] = 0x05;
        cpu.b = 1;
        let cpu_before = cpu.clone();

        cpu.process_instruction();

        assert_eq!(cpu_before.flags.z, cpu.flags.z);
        assert_ne!(cpu_before.flags.s, cpu.flags.s);
        assert_eq!(cpu_before.flags.p, cpu.flags.p);
        assert_eq!(cpu_before.flags.cy, cpu.flags.cy);
        assert_eq!(cpu_before.b, cpu.b + 1);
    }
}

