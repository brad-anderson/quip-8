#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

static FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

#[derive(Default)]
struct Quip8App {
    chip8: Chip8,
}

impl Quip8App {
    #[allow(unused_variables)]
    fn new(cc: &eframe::CreationContext<'_>, initial_rom: Option<std::path::PathBuf>) -> Self {
        // egui customizations go here
        Self {
            chip8: Chip8::new(initial_rom),
            ..Self::default()
        }
    }
}

impl eframe::App for Quip8App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            use egui::menu;
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        // …
                    }
                });
            });
        });

        egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
            ui.add_enabled_ui(self.chip8.loaded_rom.is_some(), |ui| {
                if ui.button("Step").clicked() {
                    self.chip8.emulate_cycle();
                }
                ui.label("Next opcode:");
                ui.label(format!("{:#06x}", self.chip8.next_opcode));
            });
        });

        egui::SidePanel::left("registers").show(ctx, |ui| {
            ui.heading("Registers");
            egui::Grid::new("registers").show(ui, |ui| {
                ui.label("PC");
                ui.label(self.chip8.pc.to_string());
                ui.end_row();

                ui.label("I");
                ui.label(self.chip8.i.to_string());
                ui.end_row();

                ui.label("V0");
                ui.label(self.chip8.v[0].to_string());
                ui.end_row();

                ui.label("V1");
                ui.label(self.chip8.v[1].to_string());
                ui.end_row();

                ui.label("V2");
                ui.label(self.chip8.v[2].to_string());
                ui.end_row();

                ui.label("V3");
                ui.label(self.chip8.v[3].to_string());
                ui.end_row();

                ui.label("V4");
                ui.label(self.chip8.v[4].to_string());
                ui.end_row();

                ui.label("V5");
                ui.label(self.chip8.v[5].to_string());
                ui.end_row();

                ui.label("V6");
                ui.label(self.chip8.v[6].to_string());
                ui.end_row();

                ui.label("V7");
                ui.label(self.chip8.v[7].to_string());
                ui.end_row();

                ui.label("V8");
                ui.label(self.chip8.v[8].to_string());
                ui.end_row();

                ui.label("V9");
                ui.label(self.chip8.v[9].to_string());
                ui.end_row();

                ui.label("VA");
                ui.label(self.chip8.v[10].to_string());
                ui.end_row();

                ui.label("VB");
                ui.label(self.chip8.v[11].to_string());
                ui.end_row();

                ui.label("VC");
                ui.label(self.chip8.v[12].to_string());
                ui.end_row();

                ui.label("VD");
                ui.label(self.chip8.v[13].to_string());
                ui.end_row();

                ui.label("VE");
                ui.label(self.chip8.v[14].to_string());
                ui.end_row();

                ui.label("VF");
                ui.label(self.chip8.v[15].to_string());
                ui.end_row();
            });

            ui.heading("Timers");
            egui::Grid::new("timers").show(ui, |ui| {
                ui.label("Delay");
                ui.label(self.chip8.pc.to_string());
                ui.end_row();

                ui.label("Sound");
                ui.label(self.chip8.pc.to_string());
                ui.end_row();
            });
        });

        egui::SidePanel::right("memory").show(ctx, |ui| {
            ui.heading("Memory");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello World!");
            //ui.painter().rect(ui.shrink_height_to_current)

            if let Some(file) = &self.chip8.loaded_rom {
                ui.heading(
                    file.file_name()
                        .expect("no filename")
                        .to_str()
                        .expect("filename not valid unicode"),
                );
            }
        });

        /*if self.chip8.loaded_rom.is_some() {
            self.chip8.emulate_cycle();

            if self.chip8.draw_flag {
                draw_graphics();
            }

            self.chip8.set_keys();
        }
        */
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

    next_opcode: u16,

    loaded_rom: Option<std::path::PathBuf>,
}

impl Default for Chip8 {
    fn default() -> Self {
        Self::new(None)
    }
}

impl Chip8 {
    pub fn new(rom: Option<std::path::PathBuf>) -> Self {
        let mut s = Self {
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
            next_opcode: 0,
            loaded_rom: rom.clone(),
        };
        s.memory[0..FONT_SET.len()].copy_from_slice(FONT_SET.as_slice());
        if let Some(r) = rom {
            let mut file_contents = std::fs::read(r).expect("Could not read file");
            file_contents.resize(4096 - 512, 0);
            s.memory[512..4096].copy_from_slice(file_contents.as_slice());
        }
        s.next_opcode = (s.memory[s.pc as usize] as u16) << 8 | s.memory[s.pc as usize + 1] as u16;
        s
    }

    fn emulate_cycle(&mut self) {
        // fetch opcode
        self.opcode =
            (self.memory[self.pc as usize] as u16) << 8 | self.memory[self.pc as usize + 1] as u16;

        // decode opcode
        match self.opcode & 0xF000 {
            0x6000 => {
                self.v[((self.opcode & 0x0F00) >> 8) as usize] = (self.opcode & 0x00FF) as u8;
                self.pc += 2;
            }
            0xA000 => {
                self.i = self.opcode & 0x0FFF;
                self.pc += 2;
            }
            _ => {
                std::eprintln!("Unknown opcode {:#06x}", self.opcode);
                self.pc += 2;
            }
        }

        // execute opcode

        // update timers

        self.next_opcode =
            (self.memory[self.pc as usize] as u16) << 8 | self.memory[self.pc as usize + 1] as u16;
    }
    fn set_keys(&self) {}
}

fn draw_graphics() {}

fn main() {
    let path = std::env::args().nth(1).map(std::path::PathBuf::from);

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "QUIP-8",
        options,
        Box::new(|cc| Box::new(Quip8App::new(cc, path))),
    );
}
