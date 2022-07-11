mod cpu;

fn main() {
    let mut cpu = cpu::Cpu::new();
    cpu.run();
    kdf();
}
