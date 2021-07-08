extern crate sdl2;

mod invaders;
mod cpu;
mod condition_codes;
mod op_arithmetic;
mod op_branch;
mod op_data_transfer;
mod op_logical;
mod op_special_io;
mod op_stack;
mod dissassembler;
mod interrupts;
mod helpers;

use std::io;
use std::env;
use std::thread;
use std::time::Duration;

use invaders::emulate_invaders;
use invaders::Machine;

use cpu::CPUState;
use helpers::{new_machine, generate_interrupt};

use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Rect;

const SCALE_FACTOR: i32 = 3;
const CYCLES_PER_FRAME:u64 = 4_000_000 / 60;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let (mut machine, buffer) = new_machine();

    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Intel 8080 Emulator", (224 * SCALE_FACTOR) as u32, (256 * SCALE_FACTOR) as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown {keycode: Some(Keycode::A), .. } => machine.in_port1 |= 0x20,
                Event::KeyDown {keycode: Some(Keycode::D), .. } => machine.in_port1 |= 0x40,
                Event::KeyDown {keycode: Some(Keycode::W), .. } => machine.in_port1 |= 0x10,

                Event::KeyDown {keycode: Some(Keycode::J), .. } => machine.in_port2 |= 0x20,
                Event::KeyDown {keycode: Some(Keycode::L), .. } => machine.in_port2 |= 0x40,
                Event::KeyDown {keycode: Some(Keycode::I), .. } => machine.in_port2 |= 0x10,

                Event::KeyDown {keycode: Some(Keycode::Num1), .. } => machine.in_port1 |= 0x04,
                Event::KeyDown {keycode: Some(Keycode::Num2), .. } => machine.in_port1 |= 0x02,

                Event::KeyDown {keycode: Some(Keycode::C), .. } => { println!("Pressed key"); machine.in_port1 |= 0x1 },



                Event::KeyUp {keycode: Some(Keycode::A), .. } => machine.in_port1 &= !0x20,
                Event::KeyUp {keycode: Some(Keycode::D), .. } => machine.in_port1 &= !0x40,
                Event::KeyUp {keycode: Some(Keycode::W), .. } => machine.in_port1 &= !0x10,

                Event::KeyUp {keycode: Some(Keycode::J), .. } => machine.in_port2 &= !0x20,
                Event::KeyUp {keycode: Some(Keycode::L), .. } => machine.in_port2 &= !0x40,
                Event::KeyUp {keycode: Some(Keycode::I), .. } => machine.in_port2 &= !0x10,

                Event::KeyUp {keycode: Some(Keycode::Num1), .. } => machine.in_port1 &= !0x04,
                Event::KeyUp {keycode: Some(Keycode::Num2), .. } => machine.in_port1 &= !0x02,

                Event::KeyUp {keycode: Some(Keycode::C), .. } => machine.in_port1 &= !0x1,
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        machine = half_step(machine, &mut canvas, true);
        machine = half_step(machine, &mut canvas, false);
        canvas.present();
        thread::sleep(Duration::from_millis(16));
    }
    Ok(())
}

fn half_step(mut machine: Machine, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, top_half: bool) -> Machine {
    let mut cycles_spent:u128 = 0;
    while cycles_spent < (CYCLES_PER_FRAME / 2) as u128 {
        println!("PC {:04x}", machine.cpu.pc);
        println!("Flags {}", machine.cpu.cc);
        machine = emulate_invaders(machine);
        cycles_spent += machine.cpu.cycles as u128;
    }
    //println!("{}", cycles_spent);
    redraw_screen(canvas, &machine, top_half);
    let int_enable = machine.cpu.int_enable;
    if int_enable {
        return Machine {
            cpu: generate_interrupt(machine.cpu, if top_half { 1 } else { 2 }),
            ..machine
        }
    }
    machine
}

fn redraw_screen(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, machine: &Machine, top_half: bool) {
    let width:usize = 224;
    let height:usize = 256;
    let (start_memory, start_pixel) = if top_half {
        (0x2400, 0)
    } else {
        (0x3200, 0x7000)
    };

    for offset in 0..0xE00 {
        let byte = machine.cpu.memory[start_memory + offset];

        for bit in 0..8 {
            let color: u32 = if byte & (1 << bit) == 0 {
                0x00_00_00_00
            } else {
                0xff_ff_ff_ff
            };

            let x = (start_pixel + 8 * offset + bit) / height;
            let y = height - 1 - (start_pixel + 8 * offset + bit) % height;

            if color != 0x0 {
                draw_pixel(canvas, x as i32, y as i32);
            }
        }
    }
}

fn draw_pixel(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32) {
    let width = 224*SCALE_FACTOR;
    let height = 256*SCALE_FACTOR;

    if (y > 32) & (y <= 64) {
        canvas.set_draw_color(Color::RGB(255, 0, 0));
    } else if y > 184 && y <= 240 && x >= 0 && x <= 223 {
        canvas.set_draw_color(Color::RGB(0, 255, 0));
    } else if ((y > 238) & (y <= 256) & (x >= 16)) && (x < 132) {
        canvas.set_draw_color(Color::RGB(0, 255, 0));
    } else {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
    }


    let newx = x*width/224;
    let newy = y*height/256;
    let pixel_width = ((x + 1) * width / 224) - newx;
    let pixel_height = ((y + 1) * height / 256) - newy;
    canvas.fill_rect(Rect::new(newx, newy, pixel_width as u32, pixel_height as u32));
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Read;
    use cpu::CPUState;
    use dissassembler::disassemble;
    use std::fs::File;
    use cpu::emulate_8080_op;

    #[test]
    fn test_37_410_instructions_cpu_pc() {
        let mut cpu = CPUState::new();
        let mut buffer = Vec::new();
        let mut f = File::open("invaders").unwrap();
        f.read_to_end(&mut buffer).unwrap();
        cpu.load_memory(&buffer, buffer.len());
        let mut n = 0;
        while n < 37410 {
            cpu = emulate_8080_op(cpu);
            disassemble(&buffer[cpu.pc as usize..], cpu.pc as usize);
            n += 1;
        }
        assert_eq!(cpu.pc , 0x090e)
    }

    #[test]
    fn test_39_000_instructions_cpu_pc() {
        let mut cpu = CPUState::new();
        let mut buffer = Vec::new();
        let mut f = File::open("invaders").unwrap();
        f.read_to_end(&mut buffer).unwrap();
        cpu.load_memory(&buffer, buffer.len());
        let mut n = 0;
        while n < 39000 {
            cpu = emulate_8080_op(cpu);
            disassemble(&buffer[cpu.pc as usize..], cpu.pc as usize);
            n += 1;
        }
        assert_eq!(cpu.pc , 0x1442)
    }

    #[test]
    fn test_37_410_instructions_cpu_sp() {
        let mut cpu = CPUState::new();
        let mut buffer = Vec::new();
        let mut f = File::open("invaders").unwrap();
        f.read_to_end(&mut buffer).unwrap();
        cpu.load_memory(&buffer, buffer.len());
        let mut n = 0;
        while n < 37410 {
            cpu = emulate_8080_op(cpu);
            disassemble(&buffer[cpu.pc as usize..], cpu.pc as usize);
            n += 1;
        }
        assert_eq!(cpu.sp , 0x23f8)
    }

    #[test]
    fn test_full_ops() {
        let mut cpu = CPUState::new();
        let mut buffer = Vec::new();
        let mut f = File::open("cpudiag.bin").unwrap();
        f.read_to_end(&mut buffer).unwrap();
        let buffer = {
            let mut padding = vec![0; 0x100];
            padding[0] = 0xc3;
            padding[1] = 0x00;
            padding[2] = 0x01;
            padding.append(&mut buffer);
            //padding[368] = 0x7;
            padding[0x59c] = 0xc3;
            padding[0x59d] = 0xc2;
            padding[0x59e] = 0x05;
            padding[0x319] = 0x00;
            padding[0x31a] = 0x00;
            padding[0x31b] = 0x00;
            padding[0x31c] = 0x00;
            cpu.pc = 0x100;
            padding
        };
        cpu.load_memory(&buffer, buffer.len());
        let mut n = 0;
        while n < 590 {
            cpu = emulate_8080_op(cpu);
            disassemble(&buffer[cpu.pc as usize..], cpu.pc as usize);
            n += 1;
        }
        assert_eq!(cpu.pc, 0x0688);
    }

}
