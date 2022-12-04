#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

#[derive(Default)]
struct Quip8App {}

impl Quip8App {
    #[allow(unused_variables)]
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // egui customizations go here
        Self::default()
    }
}

impl eframe::App for Quip8App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
        });
    }
}

#[allow(dead_code)]
struct Chip8 {
    opcode: u16,
    memory: [u8; 4096],
    v: [u8; 16],
    i: u16,
    pc: u16,
    gfx: [u8; 64 * 32],
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; 16],
    sp: u16,
    key: [u8; 16],

    draw_flag: bool,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            opcode: 0,
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: 0x200,
            gfx: [0; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            key: [0; 16],
            draw_flag: false,
        }
    }

    fn emulate_cycle(&self) {
        // fetch opcode
        // decode opcode
        // execute opcode

        // update timers
    }
    fn set_keys(&self) {}
}

fn draw_graphics() {}

fn main() {
    let chip8 = Chip8::new();

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "QUIP-8",
        options,
        Box::new(|cc| Box::new(Quip8App::new(cc))),
    );

    loop {
        chip8.emulate_cycle();

        if chip8.draw_flag {
            draw_graphics();
        }

        chip8.set_keys();
        break;
    }
}
