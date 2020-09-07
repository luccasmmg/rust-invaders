pub fn handle_interrupts(cpu: CPUState) -> CPUState {
    cpu
}

fn in_space_invaders(port: u8) -> u8 {
    match port {
        0 => 1,
        1 => 0,
        _ => 1,
    }
}
