extern crate sfml;

use sfml::graphics::{Color, RenderTarget, RenderWindow, Sprite, Texture, Transformable};
use sfml::window::{Event, Key, Style};

use crate::cpu::CPUState;
use crate::cpu::emulate_8080_op;
use crate::interrupts::handle_interrupts;

const SCALE: u32 = 8;

pub struct Machine {
    pub cpu: CPUState,
    pub last_timer: u64,
    pub next_interrupt: u8,
    pub which_interrupt: usize,

    pub shift_value: u16,
    pub shift_offset: u8, //SHIFT AMOUNT

    pub out_bus: u8,
    pub ports: Vec<u8>,

    pub window: RenderWindow,

}

impl Machine {
    pub fn new() -> Machine {
        let window = RenderWindow::new(
            (224 * SCALE, 256 * SCALE),
            "Rusty Invaders",
            Style::CLOSE,
            &Default::default(),
        );

        Machine {
            cpu: CPUState::new(),
            last_timer: 0,
            next_interrupt: 0,
            which_interrupt: 0,
            shift_offset: 0,
            shift_value: 0,
            out_bus: 0,
            ports: vec![0; 8],
            window,
        }
    }

    pub fn load_memory(&mut self, rom: &Vec<u8>, size: usize) {
        self.cpu.load_memory(rom, size);
    }

    fn get_frame(&self) -> &[u8] {
        &self.cpu.memory[0x2400..0x4000]
    }

    fn draw(&mut self) -> () {
        let mut texture = Texture::new(256, 224).expect("Unable to create texture");
        let mut buffer = Vec::new();
        for pixel in self.get_frame() {
            for i in 0..8 {
                let res = if (pixel & 1 << i) > 0 { 0xff } else { 0x00 };
                buffer.push(res as u8);
                buffer.push(res as u8);
                buffer.push(res as u8);
                buffer.push(255);
            }
        }

        texture.update_from_pixels(&buffer, 256, 224, 0, 0);

        let mut sprite = Sprite::with_texture(&texture);
        sprite.set_rotation(270f32);
        sprite.set_position((0f32, SCALE as f32 * 256f32));
        sprite.set_scale((SCALE as f32, SCALE as f32));

        self.window.clear(&Color::BLACK);
        self.window.draw(&sprite);
        self.window.display();
    }

}

pub fn emulate_invaders(mut machine: Machine, opcode: &[u8]) -> Machine {
    machine.draw();
    match opcode[0] {
        0xdb | 0xd3 => handle_interrupts(machine, opcode),
        _ => Machine { cpu: emulate_8080_op(machine.cpu, opcode), ..machine}
    }
}
