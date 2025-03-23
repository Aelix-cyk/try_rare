pub struct Cpu {
    regs: [u64; 32],
    pc: u64,
    bus: Bus,
}
