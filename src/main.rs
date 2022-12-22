#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use egui::{Color32, Key, Pos2, Rect, RichText, Sense, Stroke, Vec2};
use rand::prelude::*;
use std::cmp;

const FONT_START_ADDRESS: u16 = 0x0;
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

const PRIMARY_COLOR: Color32 = Color32::from_rgb(2, 238, 179);
const COMPLEMENTARY_COLOR: Color32 = Color32::from_rgb(238, 2, 61);
const ANALAGOUS1_COLOR: Color32 = Color32::from_rgb(2, 238, 61);
const ANALAGOUS2_COLOR: Color32 = Color32::from_rgb(2, 179, 238);

#[derive(Default)]
struct Quip8App {
    chip8: Chip8,
    paused: bool,
}

impl Quip8App {
    #[allow(unused_variables)]
    fn new(cc: &eframe::CreationContext<'_>, initial_rom: Option<std::path::PathBuf>) -> Self {
        // egui customizations go here
        Self {
            chip8: Chip8::new(initial_rom),
            paused: true,
            ..Self::default()
        }
    }
}

impl eframe::App for Quip8App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let previous_keys = self.chip8.keys;
        self.chip8.keys = ctx.input().keys_down.contains(&Key::Num1) as u16
            | (ctx.input().keys_down.contains(&Key::Num2) as u16) << 1
            | (ctx.input().keys_down.contains(&Key::Num3) as u16) << 2
            | (ctx.input().keys_down.contains(&Key::Num4) as u16) << 3
            | (ctx.input().keys_down.contains(&Key::Q) as u16) << 4
            | (ctx.input().keys_down.contains(&Key::W) as u16) << 5
            | (ctx.input().keys_down.contains(&Key::E) as u16) << 6
            | (ctx.input().keys_down.contains(&Key::R) as u16) << 7
            | (ctx.input().keys_down.contains(&Key::A) as u16) << 8
            | (ctx.input().keys_down.contains(&Key::S) as u16) << 9
            | (ctx.input().keys_down.contains(&Key::D) as u16) << 10
            | (ctx.input().keys_down.contains(&Key::F) as u16) << 11
            | (ctx.input().keys_down.contains(&Key::Z) as u16) << 12
            | (ctx.input().keys_down.contains(&Key::X) as u16) << 13
            | (ctx.input().keys_down.contains(&Key::C) as u16) << 14
            | (ctx.input().keys_down.contains(&Key::V) as u16) << 15;

        self.chip8.key_pressed = None;
        let keys_pressed_since = (previous_keys & self.chip8.keys) ^ self.chip8.keys;
        if keys_pressed_since != 0 {
            // This could be bit twiddled out
            let mut b = 0;
            self.chip8.key_pressed = loop {
                if keys_pressed_since & (1 << b) != 0 {
                    break Some(b);
                }
                b += 1;
                if b > 15 {
                    break None;
                }
            };
        }

        let mut run_cycles = if !self.paused { 1 } else { 0 };

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            use egui::menu;
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        // …
                    }
                });
                ui.separator();
                ui.add_enabled_ui(!self.paused, |ui| {
                    if ui.button("Pause").clicked() {
                        self.paused = !self.paused;
                    }
                });
                ui.add_enabled_ui(self.paused, |ui| {
                    if ui.button("Run").clicked() {
                        self.paused = !self.paused;
                    }
                    if ui.button("Step").clicked() {
                        run_cycles = 1;
                    }
                    if ui.button("Step 5").clicked() {
                        run_cycles = 5;
                    }
                });
            });
        });

        egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
            ui.add_enabled_ui(self.chip8.loaded_rom.is_some(), |ui| {
                ui.horizontal(|ui| {
                    egui::Grid::new("registers").show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("PC").monospace());
                            ui.label(
                                RichText::new(format!("{:03X}", self.chip8.pc))
                                    .monospace()
                                    .color(ANALAGOUS2_COLOR),
                            );
                            ui.separator();
                        });

                        for v in self.chip8.v[0..8].iter().enumerate() {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(format!("V{:X}", v.0)).monospace());
                                ui.label(
                                    RichText::new(format!("{:02X}", v.1))
                                        .monospace()
                                        .color(ANALAGOUS2_COLOR),
                                );
                                ui.separator();
                            });
                        }
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("DELAY").monospace());
                            ui.label(
                                RichText::new(format!("{:02X}", self.chip8.delay_timer))
                                    .monospace()
                                    .color(ANALAGOUS2_COLOR),
                            );
                            ui.separator();
                        });
                        ui.end_row();

                        ui.horizontal(|ui| {
                            ui.label(RichText::new("I ").monospace());
                            ui.label(
                                RichText::new(format!("{:03X}", self.chip8.i))
                                    .monospace()
                                    .color(ANALAGOUS2_COLOR),
                            );
                            ui.separator();
                        });

                        for v in self.chip8.v[8..].iter().enumerate() {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(format!("V{:X}", v.0 + 8)).monospace());
                                ui.label(
                                    RichText::new(format!("{:02X}", v.1))
                                        .monospace()
                                        .color(ANALAGOUS2_COLOR),
                                );
                                ui.separator();
                            });
                        }
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("SOUND").monospace());
                            ui.label(
                                RichText::new(format!("{:02X}", self.chip8.sound_timer))
                                    .monospace()
                                    .color(ANALAGOUS2_COLOR),
                            );
                            ui.separator();
                            ui.end_row();
                        });
                        ui.end_row();
                    });
                });
                /*let raw_opcode = (self.chip8.memory[self.chip8.pc as usize] as u16) << 8
                    | self.chip8.memory[(self.chip8.pc + 1) as usize] as u16;
                ui.label(format!(
                    "Next opcode: {:#06X} - {}",
                    raw_opcode,
                    Opcode::decode(raw_opcode)
                        .map_or("Unknown opcode".to_string(), |o| o.describe(&self.chip8))
                ));
                */
                /*
                if let Some(file) = &self.chip8.loaded_rom {
                    ui.heading(
                        file.file_name()
                            .expect("no filename")
                            .to_str()
                            .expect("filename not valid unicode"),
                    );
                }*/
            });
        });

        egui::SidePanel::right("memory").show(ctx, |ui| {
            ui.heading("Keys");
            let key_color = |key: u32| {
                if self.chip8.keys & (1 << key) != 0 {
                    COMPLEMENTARY_COLOR
                } else {
                    Color32::GRAY
                }
            };
            egui::Grid::new("keys").show(ui, |ui| {
                ui.colored_label(key_color(0), "1");
                ui.colored_label(key_color(1), "2");
                ui.colored_label(key_color(2), "3");
                ui.colored_label(key_color(3), "4");
                ui.end_row();

                ui.colored_label(key_color(4), "Q");
                ui.colored_label(key_color(5), "W");
                ui.colored_label(key_color(6), "E");
                ui.colored_label(key_color(7), "R");
                ui.end_row();

                ui.colored_label(key_color(8), "A");
                ui.colored_label(key_color(9), "S");
                ui.colored_label(key_color(10), "D");
                ui.colored_label(key_color(11), "F");
                ui.end_row();

                ui.colored_label(key_color(12), "Z");
                ui.colored_label(key_color(13), "X");
                ui.colored_label(key_color(14), "C");
                ui.colored_label(key_color(15), "V");
                ui.end_row();
            });
            ui.separator();
            ui.heading("Instructions");
            for i in 0..12 {
                let pc = (self.chip8.pc as i64 + (i as i16 - 3) as i64 * 2) as usize;
                let raw_opcode =
                    (self.chip8.memory[pc] as u16) << 8 | self.chip8.memory[pc + 1] as u16;
                let opcode_maybe = Opcode::decode(raw_opcode);
                if let Ok(opcode) = opcode_maybe {
                    ui.label(
                        RichText::new(format!(
                            "{}{:03X} {:5} {:3} {:2} {:2}",
                            if pc == self.chip8.pc as usize {
                                "\u{2794}"
                            } else {
                                " "
                            },
                            pc,
                            opcode.mnemonic(),
                            opcode
                                .operand(0)
                                .map_or("".to_owned(), |o| format!("{:3X}", o)),
                            opcode
                                .operand(1)
                                .map_or("".to_owned(), |o| format!("{:2X}", o)),
                            opcode
                                .operand(2)
                                .map_or("".to_owned(), |o| format!("{:2X}", o)),
                        ))
                        .color(ANALAGOUS2_COLOR)
                        .monospace(),
                    );
                } else {
                    ui.label(
                        RichText::new(format!(
                            "{}{:03X} UNKNOWN {:#06X}",
                            if pc == self.chip8.pc as usize {
                                "\u{2794}"
                            } else {
                                " "
                            },
                            pc,
                            raw_opcode
                        ))
                        .monospace(),
                    );
                }
            }
            ui.separator();
            ui.heading("Stack");
            for i in 0..self.chip8.sp {
                ui.label(
                    RichText::new(format!(
                        "{:03X}",
                        self.chip8.stack[(self.chip8.sp - i) as usize]
                    ))
                    .color(ANALAGOUS2_COLOR)
                    .monospace(),
                );
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let painter_size = ui.available_size();
            let (res, painter) = ui.allocate_painter(painter_size, Sense::hover());

            let display_size = if res.rect.width() / 2.0 > res.rect.height() {
                Vec2::new(res.rect.height() * 2.0, res.rect.height())
            } else {
                Vec2::new(res.rect.width(), res.rect.width() / 2.0)
            };
            let display_rect = Rect::from_center_size(res.rect.center(), display_size);
            painter.rect_filled(display_rect, 0.0, Color32::from_rgb(5, 10, 5));
            for (row, row_data) in self.chip8.gfx.iter().enumerate() {
                for col in 0..64 {
                    if row_data & (1 << (64 - col - 1)) > 0 {
                        painter.rect_filled(
                            Rect {
                                min: Pos2 {
                                    x: display_rect.left()
                                        + display_rect.width() / 64.0 * col as f32,
                                    y: display_rect.top()
                                        + display_rect.height() / 32.0 * row as f32,
                                },
                                max: Pos2 {
                                    x: display_rect.left()
                                        + display_rect.width() / 64.0 * (col + 1) as f32,
                                    y: display_rect.top()
                                        + display_rect.height() / 32.0 * (row + 1) as f32,
                                },
                            },
                            0.0,
                            PRIMARY_COLOR,
                        );
                    } else {
                        painter.rect_stroke(
                            Rect {
                                min: Pos2 {
                                    x: display_rect.left()
                                        + display_rect.width() / 64.0 * col as f32,
                                    y: display_rect.top()
                                        + display_rect.height() / 32.0 * row as f32,
                                },
                                max: Pos2 {
                                    x: display_rect.left()
                                        + display_rect.width() / 64.0 * (col + 1) as f32,
                                    y: display_rect.top()
                                        + display_rect.height() / 32.0 * (row + 1) as f32,
                                },
                            },
                            0.0,
                            Stroke::new(1.0, Color32::from_rgb(40, 40, 40)),
                        );
                    }
                }
            }
        });

        for _ in 0..run_cycles {
            self.chip8.emulate_cycle();
        }

        if run_cycles > 0 {
            ctx.request_repaint();
        }
    }
}

type Address = u16;
type RegisterAddress = u8;
type Literal = u8;

struct Chip8 {
    opcode: u16,
    memory: [u8; 4096],
    v: [u8; 16],
    i: u16,
    pc: Address,
    gfx: [u64; 32],
    delay_timer: u8,
    sound_timer: u8,
    stack: [Address; 16],
    sp: u16,
    keys: u16,
    key_pressed: Option<u8>,

    loaded_rom: Option<std::path::PathBuf>,
}

impl Default for Chip8 {
    fn default() -> Self {
        Self::new(None)
    }
}

enum Opcode {
    // 0x0NNN - Call
    // Calls machine code routine (RCA 1802 for COSMAC VIP) at address NNN. Not necessary for most ROMs.
    SYS(Address),

    // 0x00E0 - Display
    // C Pseudo: disp_clear()
    // Clears the screen.
    CLR,

    // 0x00EE - Flow
    // C Pseudo: return;
    // Returns from a subroutine.
    RTS,

    // 0x1NNN - Flow
    // C Pseudo: goto NNN;
    // Jumps to address NNN.
    JUMP(Address),

    // 0x2NNN - Flow
    // C Pseudo: *(0xNNN)()
    // Calls subroutine at NNN.
    CALL(Address),

    // 0x3XNN - Cond
    // C Pseudo: if (Vx == NN)
    // Skips the next instruction if VX equals NN (usually the next instruction
    // is a jump to skip a code block).
    SKE((RegisterAddress, Literal)),

    // 0x4XNN - Cond
    // C Pseudo: if (Vx != NN)
    // Skips the next instruction if VX does not equal NN (usually the next
    // instruction is a jump to skip a code block).
    SKNE((RegisterAddress, Literal)),

    // 0x5XY0 - Cond
    // C Pseudo: if (Vx == Vy)
    // Skips the next instruction if VX equals VY (usually the next instruction
    // is a jump to skip a code block).
    SKRE((RegisterAddress, RegisterAddress)),

    // 0x6XNN - Const
    // C Pseudo: Vx = NN
    // Sets VX to NN.
    LOAD((RegisterAddress, Literal)),

    // 0x7XNN - Const
    // C Pseudo: Vx += NN
    // Adds NN to VX (carry flag is not changed).
    ADD((RegisterAddress, Literal)),

    // 0x8XY0 - Assig
    // C Pseudo: Vx = Vy
    // Sets VX to the value of VY.
    MOVE((RegisterAddress, RegisterAddress)),

    // 0x8XY1 - BitOp
    // C Pseudo: Vx |= Vy
    // Sets VX to VX or VY. (bitwise OR operation)
    OR((RegisterAddress, RegisterAddress)),

    // 0x8XY2 - BitOp
    // C Pseudo: Vx &= Vy
    // Sets VX to VX and VY. (bitwise AND operation)
    AND((RegisterAddress, RegisterAddress)),

    // 0x8XY3[a] - BitOp
    // C Pseudo: Vx ^= Vy
    // Sets VX to VX xor VY.
    XOR((RegisterAddress, RegisterAddress)),

    // 0x8XY4 - Math
    // C Pseudo: Vx += Vy
    // Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there
    // is not.
    ADDR((RegisterAddress, RegisterAddress)),

    // 0x8XY5 - Math
    // C Pseudo: Vx -= Vy
    // VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1
    // when there is not.
    SUB((RegisterAddress, RegisterAddress)),

    // 0x8XY6 - BitOp
    // C Pseudo: Vx >>= 1
    // Stores the least significant bit of VX in VF and then shifts VX to the
    // right by 1.
    // Better Name?
    SHR((RegisterAddress, RegisterAddress)),

    // 0x8XY7 - Math
    // C Pseudo: Vx = Vy - Vx
    // Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when
    //  there is not.
    // Better Name?
    RSUB((RegisterAddress, RegisterAddress)),

    // 0x8XYE - BitOp
    // C Pseudo: Vx <<= 1
    // Stores the most significant bit of VX in VF and then shifts VX to the
    // left by 1.
    // Better Name?
    SHL((RegisterAddress, RegisterAddress)),

    // 0x9XY0 - Cond
    // C Pseudo: if (Vx != Vy)
    // Skips the next instruction if VX does not equal VY. (Usually the next
    // instruction is a jump to skip a code block);
    SKRNE((RegisterAddress, RegisterAddress)),

    // 0xANNN - MEM
    // C Pseudo: I = NNN
    // Sets I to the address NNN.
    LOADI(Address),

    // 0xBNNN - Flow
    // C Pseudo: PC = V0 + NNN
    // Jumps to the address NNN plus V0.
    JUMPI(Address),

    // 0xCXNN - Rand
    // C Pseudo: Vx = rand() & NN
    // Sets VX to the result of a bitwise and operation on a random number
    // (Typically: 0 to 255) and NN.
    RAND((RegisterAddress, Literal)),

    // 0xDXYN - Display
    // C Pseudo: draw(Vx, Vy, N)
    // Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and
    // a height of N pixels. Each row of 8 pixels is read as bit-coded starting
    // from memory location I; I value does not change after the execution of
    // this instruction. As described above, VF is set to 1 if any screen pixels
    // are flipped from set to unset when the sprite is drawn, and to 0 if that
    // does not happen.
    DRAW((RegisterAddress, RegisterAddress, Literal)),

    // 0xEX9E - KeyOp
    // C Pseudo: if (key() == Vx)
    // Skips the next instruction if the key stored in VX is pressed (usually
    // the next instruction is a jump to skip a code block).
    SKPR(RegisterAddress),

    // 0xEXA1 - KeyOp
    // C Pseudo: if (key() != Vx)
    // Skips the next instruction if the key stored in VX is not pressed
    // (usually the next instruction is a jump to skip a code block).
    SKUP(RegisterAddress),

    // 0xFX07 - Timer
    // C Pseudo: Vx = get_delay()
    // Sets VX to the value of the delay timer.
    MOVED(RegisterAddress),

    // 0xFX0A - KeyOp
    // C Pseudo: Vx = get_key()
    // A key press is awaited, and then stored in VX (blocking operation, all
    // instruction halted until next key event).
    KEYD(RegisterAddress),

    // 0xFX15 - Timer
    // C Pseudo: delay_timer(Vx)
    // Sets the delay timer to VX.
    LOADD(RegisterAddress),

    // 0xFX18 - Sound
    // C Pseudo: sound_timer(Vx)
    // Sets the sound timer to VX.
    LOADS(RegisterAddress),

    // 0xFX1E - MEM
    // C Pseudo: I += Vx
    // Adds VX to I. VF is not affected.[c]
    ADDI(RegisterAddress),

    // 0xFX29 - MEM
    // C Pseudo: I = sprite_addr[Vx]
    // Sets I to the location of the sprite for the character in VX. Characters
    //  0-F (in hexadecimal) are represented by a 4x5 font.
    LDSPR(RegisterAddress),

    // 0xFX33 - BCD
    // C Pseudo: set_BCD(Vx) *(I+0) = BCD(3); *(I+1) = BCD(2); *(I+2) = BCD(1);
    // Stores the binary-coded decimal representation of VX, with the hundreds
    // digit in memory at location in I, the tens digit at location I+1, and
    // the ones digit at location I+2.
    BCD(RegisterAddress),

    // 0xFX55 - MEM
    // C Pseudo: reg_dump(Vx, &I)
    // Stores from V0 to VX (including VX) in memory, starting at address I.
    // The offset from I is increased by 1 for each value written, but I itself
    // is left unmodified.[d]
    STORE(RegisterAddress),

    // 0xFX65 - MEM
    // C Pseudo: reg_load(Vx, &I)
    // Fills from V0 to VX (including VX) with values from memory, starting at
    // address I. The offset from I is increased by 1 for each value read, but
    // I itself is left unmodified.[d]
    READ(RegisterAddress),
}

struct UnknownOpcode;

impl Opcode {
    pub fn decode(raw_opcode: u16) -> Result<Opcode, UnknownOpcode> {
        match raw_opcode & 0xF000 {
            0x0000 => match raw_opcode {
                0x00E0 => Ok(Opcode::CLR),
                0x00EE => Ok(Opcode::RTS),
                _ => Ok(Opcode::SYS((raw_opcode & 0x0FFF) as Address)),
            },
            0x1000 => Ok(Opcode::JUMP((raw_opcode & 0x0FFF) as Address)),
            0x2000 => Ok(Opcode::CALL((raw_opcode & 0x0FFF) as Address)),
            0x3000 => Ok(Opcode::SKE((
                ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                (raw_opcode & 0x00FF) as Literal,
            ))),
            0x4000 => Ok(Opcode::SKNE((
                ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                (raw_opcode & 0x00FF) as Literal,
            ))),
            0x5000 => Ok(Opcode::SKRE((
                ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                ((raw_opcode & 0x00F0) >> 4) as RegisterAddress,
            ))),
            0x6000 => Ok(Opcode::LOAD((
                ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                (raw_opcode & 0x00FF) as Literal,
            ))),
            0x7000 => Ok(Opcode::ADD((
                ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                (raw_opcode & 0x00FF) as Literal,
            ))),
            0x8000 => match raw_opcode & 0x000F {
                0x0000 => Ok(Opcode::MOVE((
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                    ((raw_opcode & 0x00F0) >> 4) as RegisterAddress,
                ))),
                0x0001 => Ok(Opcode::OR((
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                    ((raw_opcode & 0x00F0) >> 4) as RegisterAddress,
                ))),
                0x0002 => Ok(Opcode::AND((
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                    ((raw_opcode & 0x00F0) >> 4) as RegisterAddress,
                ))),
                0x0003 => Ok(Opcode::XOR((
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                    ((raw_opcode & 0x00F0) >> 4) as RegisterAddress,
                ))),
                0x0004 => Ok(Opcode::ADDR((
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                    ((raw_opcode & 0x00F0) >> 4) as RegisterAddress,
                ))),
                0x0005 => Ok(Opcode::SUB((
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                    ((raw_opcode & 0x00F0) >> 4) as RegisterAddress,
                ))),
                0x0006 => Ok(Opcode::SHR((
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                    ((raw_opcode & 0x00F0) >> 4) as RegisterAddress,
                ))),
                0x0007 => Ok(Opcode::RSUB((
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                    ((raw_opcode & 0x00F0) >> 4) as RegisterAddress,
                ))),
                0x000E => Ok(Opcode::SHL((
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                    ((raw_opcode & 0x00F0) >> 4) as RegisterAddress,
                ))),
                _ => Err(UnknownOpcode),
            },
            0x9000 => Ok(Opcode::SKRNE((
                ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                ((raw_opcode & 0x00F0) >> 4) as RegisterAddress,
            ))),
            0xA000 => Ok(Opcode::LOADI((raw_opcode & 0x0FFF) as Address)),
            0xB000 => Ok(Opcode::JUMPI((raw_opcode & 0x0FFF) as Address)),
            0xC000 => Ok(Opcode::RAND((
                ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                (raw_opcode & 0x00FF) as Literal,
            ))),
            0xD000 => Ok(Opcode::DRAW((
                ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                ((raw_opcode & 0x00F0) >> 4) as RegisterAddress,
                (raw_opcode & 0x000F) as Literal,
            ))),
            0xE000 => match raw_opcode & 0x00FF {
                0x009E => Ok(Opcode::SKPR(
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                )),
                0x00A1 => Ok(Opcode::SKUP(
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                )),
                _ => Err(UnknownOpcode),
            },
            0xF000 => match raw_opcode & 0x00FF {
                0x0007 => Ok(Opcode::MOVED(
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                )),
                0x000A => Ok(Opcode::KEYD(
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                )),
                0x0015 => Ok(Opcode::LOADD(
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                )),
                0x0018 => Ok(Opcode::LOADS(
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                )),
                0x001E => Ok(Opcode::ADDI(
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                )),
                0x0029 => Ok(Opcode::LDSPR(
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                )),
                0x0033 => Ok(Opcode::BCD(((raw_opcode & 0x0F00) >> 8) as RegisterAddress)),
                0x0055 => Ok(Opcode::STORE(
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                )),
                0x0065 => Ok(Opcode::READ(
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                )),
                _ => Err(UnknownOpcode),
            },

            _ => Err(UnknownOpcode),
        }
    }

    fn mnemonic(&self) -> &'static str {
        match self {
            Opcode::SYS(_address) => "SYS",
            Opcode::CLR => "CLR",
            Opcode::RTS => "RTS",
            Opcode::JUMP(_address) => "JUMP",
            Opcode::CALL(_address) => "CALL",
            Opcode::SKE((_register, _literal)) => "SKE",
            Opcode::SKNE((_register, _literal)) => "SKNE",
            Opcode::SKRE((_register_x, _register_y)) => "SKRE",
            Opcode::LOAD((_register, _literal)) => "LOAD",
            Opcode::ADD((_register, _literal)) => "ADD",
            Opcode::MOVE((_register_x, _register_y)) => "MOVE",
            Opcode::OR((_register_x, _register_y)) => "OR",
            Opcode::AND((_register_x, _register_y)) => "AND",
            Opcode::XOR((_register_x, _register_y)) => "XOR",
            Opcode::ADDR((_register_x, _register_y)) => "ADDR",
            Opcode::SUB((_register_x, _register_y)) => "SUB",
            Opcode::SHR((_register_x, _register_y)) => "SHR",
            Opcode::RSUB((_register_x, _register_y)) => "RSUB",
            Opcode::SHL((_register_x, _register_y)) => "SHL",
            Opcode::SKRNE((_register_x, _register_y)) => "SKRNE",
            Opcode::LOADI(_address) => "LOADI",
            Opcode::JUMPI(_address) => "JUMPI",
            Opcode::RAND((_register, _literal)) => "RAND",
            Opcode::DRAW((_register_x, _register_y, _literal)) => "DRAW",
            Opcode::SKPR(_register) => "SKPR",
            Opcode::SKUP(_register) => "SKUP",
            Opcode::MOVED(_register) => "MOVED",
            Opcode::KEYD(_register) => "KEYD",
            Opcode::LOADD(_register) => "LOADD",
            Opcode::LOADS(_register) => "LOADS",
            Opcode::ADDI(_register) => "ADDI",
            Opcode::LDSPR(_register) => "LDSPR",
            Opcode::BCD(_register) => "BCD",
            Opcode::STORE(_register) => "STORE",
            Opcode::READ(_register) => "READ",
        }
    }

    fn operand(&self, i: u8) -> Option<u16> {
        let operands = match self {
            Opcode::SYS(address) => (Some(*address), None, None),
            Opcode::CLR => (None, None, None),
            Opcode::RTS => (None, None, None),
            Opcode::JUMP(address) => (Some(*address), None, None),
            Opcode::CALL(address) => (Some(*address), None, None),
            Opcode::SKE((register, literal)) => (Some(*register as u16), Some(*literal), None),
            Opcode::SKNE((register, literal)) => (Some(*register as u16), Some(*literal), None),
            Opcode::SKRE((register_x, register_y)) => {
                (Some(*register_x as u16), Some(*register_y), None)
            }
            Opcode::LOAD((register, literal)) => (Some(*register as u16), Some(*literal), None),
            Opcode::ADD((register, literal)) => (Some(*register as u16), Some(*literal), None),
            Opcode::MOVE((register_x, register_y)) => {
                (Some(*register_x as u16), Some(*register_y), None)
            }
            Opcode::OR((register_x, register_y)) => {
                (Some(*register_x as u16), Some(*register_y), None)
            }
            Opcode::AND((register_x, register_y)) => {
                (Some(*register_x as u16), Some(*register_y), None)
            }
            Opcode::XOR((register_x, register_y)) => {
                (Some(*register_x as u16), Some(*register_y), None)
            }
            Opcode::ADDR((register_x, register_y)) => {
                (Some(*register_x as u16), Some(*register_y), None)
            }
            Opcode::SUB((register_x, register_y)) => {
                (Some(*register_x as u16), Some(*register_y), None)
            }
            Opcode::SHR((register_x, register_y)) => {
                (Some(*register_x as u16), Some(*register_y), None)
            }
            Opcode::RSUB((register_x, register_y)) => {
                (Some(*register_x as u16), Some(*register_y), None)
            }
            Opcode::SHL((register_x, register_y)) => {
                (Some(*register_x as u16), Some(*register_y), None)
            }
            Opcode::SKRNE((register_x, register_y)) => {
                (Some(*register_x as u16), Some(*register_y), None)
            }
            Opcode::LOADI(address) => (Some(*address), None, None),
            Opcode::JUMPI(address) => (Some(*address), None, None),
            Opcode::RAND((register, literal)) => (Some(*register as u16), Some(*literal), None),
            Opcode::DRAW((register_x, register_y, literal)) => {
                (Some(*register_x as u16), Some(*register_y), Some(*literal))
            }
            Opcode::SKPR(register) => (Some(*register as u16), None, None),
            Opcode::SKUP(register) => (Some(*register as u16), None, None),
            Opcode::MOVED(register) => (Some(*register as u16), None, None),
            Opcode::KEYD(register) => (Some(*register as u16), None, None),
            Opcode::LOADD(register) => (Some(*register as u16), None, None),
            Opcode::LOADS(register) => (Some(*register as u16), None, None),
            Opcode::ADDI(register) => (Some(*register as u16), None, None),
            Opcode::LDSPR(register) => (Some(*register as u16), None, None),
            Opcode::BCD(register) => (Some(*register as u16), None, None),
            Opcode::STORE(register) => (Some(*register as u16), None, None),
            Opcode::READ(register) => (Some(*register as u16), None, None),
        };

        match i {
            0 => operands.0,
            1 => operands.1.map_or(None, |o| Some(o as u16)),
            2 => operands.2.map_or(None, |o| Some(o as u16)),
            _ => None,
        }
    }

    fn describe(&self, chip8: &Chip8) -> String {
        match self {
            Opcode::SYS(address) => format!(
                "Call machine code routine (RCA 1802 for COSMAC VIP) at address {address:#05X}."),
            Opcode::CLR => format!("Clear the screen"),
            Opcode::RTS => format!("Return from subroutine (pop the stack)"),
            Opcode::JUMP(address) => format!("Jump to address {address:#05X}"),
            Opcode::CALL(address) => format!("Calls subroutine at {address:#05X} (push on the stack)"),
            Opcode::SKE((register, literal)) => format!(
                "Skips the next instruction if V{register:X} equals {literal:#04X}"),
            Opcode::SKNE((register, literal)) => format!(
                "Skips the next instruction if V{register:X} does not equal {literal:#04X}"),
            Opcode::SKRE((register_x, register_y)) => format!("Skips the next instruction if V{register_x:X} equals V{register_y:X}"),
            Opcode::LOAD((register, literal)) => format!("Sets V{register:X} to {literal:#04X}"),
            Opcode::ADD((register, literal)) => format!("Adds {literal:#04X} to V{register:X} (carry flag is not changed)"),
            Opcode::MOVE((register_x, register_y)) => format!("Sets V{register_x:X} to the value of V{register_y}"),
            Opcode::OR((register_x, register_y)) => format!("Sets V{register_x:X} to V{register_x:X} or V{register_y:X}. (bitwise OR operation"),
            Opcode::AND((register_x, register_y)) => format!("Sets V{register_x:X} to V{register_x:X} and V{register_y:X}. (bitwise AND operation"),
            Opcode::XOR((register_x, register_y)) => format!("Sets V{register_x:X} to V{register_x:X} xor V{register_y:X}."),
            Opcode::ADDR((register_x, register_y)) => format!("Adds V{register_y:X} to V{register_x:X}. VF is set to 1 when there's a carry, and to 0 when there is not."),
            Opcode::SUB((register_x, register_y)) => format!("V{register_y:X} ({}) is subtracted from V{register_x:X} ({}). VF is set to 0 when there's a borrow, and 1 when there is not.", chip8.v[*register_y as usize], chip8.v[*register_x as usize]),
            Opcode::SHR((register_x, _register_y)) => format!("Stores the least significant bit of V{register_x:X} in VF and then shifts V{register_x:X} to the right by 1."),
            Opcode::RSUB((register_x, register_y)) => format!("Sets V{register_x:X} to V{register_y:X} minus V{register_x:X}. VF is set to 0 when there's a borrow, and 1 when there is not."),
            Opcode::SHL((register_x, _register_y)) => format!("Stores the most significant bit of V{register_x:X} in VF and then shifts V{register_x:X} to the left by 1"),
            Opcode::SKRNE((register_x, register_y)) => format!("Skips the next instruction if V{register_x:X} does not equal V{register_y:X}"),
            Opcode::LOADI(address) => format!("Sets I to the address {address:#05X}"),
            Opcode::JUMPI(address) => format!("Jumps to the address {address:#05X} plus V0"),
            Opcode::RAND((register, literal)) => format!("Sets V{register:X} to the result of a bitwise and operation on a random number (Typically: 0 to 255) and {literal:#04X}"),
            Opcode::DRAW((register_x, register_y, literal)) => format!("Draws an 8x{literal} sprite at coordinate (V{register_x:X} ({}), V{register_y:X} ({})).", chip8.v[*register_x as usize], chip8.v[*register_y as usize]),
            Opcode::SKPR(register) => format!("Skips the next instruction if the key stored in V{register:X} is pressed (usually the next instruction is a jump to skip a code block)"),
            Opcode::SKUP(register) => format!("Skips the next instruction if the key stored in V{register:X} is not pressed"),
            Opcode::MOVED(register) => format!("Sets V{register:X} to the value of the delay timer"),
            Opcode::KEYD(register) => format!("A key press is awaited, and then stored in V{register:X} (blocking operation, all instruction halted until next key event)"),
            Opcode::LOADD(register) => format!("Sets the delay timer to V{register:X}"),
            Opcode::LOADS(register) => format!("Sets the sound timer to V{register:X}"),
            Opcode::ADDI(register) => format!("Adds V{register:X} to I. VF is not affected."),
            Opcode::LDSPR(register) => format!("Sets I to the location of the sprite for the character in V{register:X}. Characters 0-F (in hexadecimal) are represented by a 4x5 font."),
            Opcode::BCD(register) => format!("Stores the binary-coded decimal representation of V{register:X}, with the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2."),
            Opcode::STORE(register) => format!("Stores from V0 to V{register:X} (including V{register:X}) in memory, starting at address I"),
            Opcode::READ(register) => format!("Fills from V0 to V{register:X} (including V{register:X}) with values from memory, starting at address I"),
        }
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
            gfx: [0; 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            keys: 0,
            key_pressed: None,
            loaded_rom: rom.clone(),
        };
        s.memory[0..FONT_SET.len()].copy_from_slice(FONT_SET.as_slice());
        if let Some(r) = rom {
            let mut file_contents = std::fs::read(r).expect("Could not read file");
            file_contents.resize(4096 - 512, 0);
            s.memory[512..].copy_from_slice(file_contents.as_slice());
        }
        s
    }

    fn emulate_cycle(&mut self) {
        // fetch opcode
        self.opcode =
            (self.memory[self.pc as usize] as u16) << 8 | self.memory[self.pc as usize + 1] as u16;
        self.pc += 2;

        // decode opcode
        match Opcode::decode(self.opcode) {
            Ok(decoded_opcode) => match decoded_opcode {
                Opcode::SYS(_address) => {
                    std::eprintln!("Unimplemented opcode {:#06X}", self.opcode);
                } //Calls machine code routine (RCA 1802 for COSMAC VIP) at address NNN. Not necessary for most ROMs.
                Opcode::CLR => {
                    self.gfx = [0; 32];
                } // 	disp_clear() 	Clears the screen.
                Opcode::RTS => {
                    self.pc = self.stack[self.sp as usize];
                    self.sp -= 1;
                } //return; 	Returns from a subroutine.
                Opcode::JUMP(address) => {
                    self.pc = address;
                } //goto NNN; 	Jumps to address NNN.
                Opcode::CALL(address) => {
                    self.sp += 1;
                    self.stack[self.sp as usize] = self.pc;
                    self.pc = address;
                } //*(0xNNN)() 	Calls subroutine at NNN.
                Opcode::SKE((register, literal)) => {
                    if self.v[register as usize] == literal {
                        self.pc += 2;
                    }
                } //if (Vx == NN) 	Skips the next instruction if VX equals NN (usually the next instruction is a jump to skip a code block).
                Opcode::SKNE((register, literal)) => {
                    if self.v[register as usize] != literal {
                        self.pc += 2;
                    }
                } //if (Vx != NN) 	Skips the next instruction if VX does not equal NN (usually the next instruction is a jump to skip a code block).
                Opcode::SKRE((register_x, register_y)) => {
                    if self.v[register_x as usize] == self.v[register_y as usize] {
                        self.pc += 2;
                    }
                } //if (Vx == Vy) 	Skips the next instruction if VX equals VY (usually the next instruction is a jump to skip a code block).
                Opcode::LOAD((register, literal)) => {
                    self.v[register as usize] = literal;
                }
                Opcode::ADD((register, literal)) => {
                    self.v[register as usize] =
                        (self.v[register as usize] as u16 + literal as u16) as u8;
                } //Vx += NN 	Adds NN to VX (carry flag is not changed).
                Opcode::MOVE((register_x, register_y)) => {
                    self.v[register_x as usize] = self.v[register_y as usize];
                } //Vx = Vy 	Sets VX to the value of VY.
                Opcode::OR((register_x, register_y)) => {
                    self.v[register_x as usize] |= self.v[register_y as usize];
                } // Vx |= Vy 	Sets VX to VX or VY. (bitwise OR operation)
                Opcode::AND((register_x, register_y)) => {
                    self.v[register_x as usize] &= self.v[register_y as usize];
                } //Vx &= Vy 	Sets VX to VX and VY. (bitwise AND operation)
                Opcode::XOR((register_x, register_y)) => {
                    self.v[register_x as usize] ^= self.v[register_y as usize];
                } // Vx ^= Vy 	Sets VX to VX xor VY.
                Opcode::ADDR((register_x, register_y)) => {
                    let (result, overflow) =
                        self.v[register_x as usize].overflowing_add(self.v[register_y as usize]);
                    self.v[register_x as usize] = result;
                    self.v[0xF] = overflow as u8;
                } // Vx += Vy 	Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there is not.
                Opcode::SUB((register_x, register_y)) => {
                    let (result, overflow) =
                        self.v[register_x as usize].overflowing_sub(self.v[register_y as usize]);
                    self.v[register_x as usize] = result;
                    self.v[0xF] = overflow as u8;
                } // Vx -= Vy 	VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there is not.
                Opcode::SHR((register_x, register_y)) => {
                    self.v[0xF] = self.v[register_y as usize] & 1;
                    self.v[register_x as usize] = self.v[register_y as usize] >> 1;
                } // Vx >>= 1 	Stores the least significant bit of VX in VF and then shifts VX to the right by 1.[b]
                Opcode::RSUB((register_x, register_y)) => {
                    self.v[register_x as usize] =
                        self.v[register_y as usize] - self.v[register_x as usize];
                } // Vx = Vy - Vx 	Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there is not.
                Opcode::SHL((register_x, register_y)) => {
                    self.v[0xF] = self.v[register_y as usize] & 0xF0;
                    self.v[register_x as usize] = self.v[register_y as usize] << 1;
                } // Vx <<= 1 	Stores the most significant bit of VX in VF and then shifts VX to the left by 1.[b]
                Opcode::SKRNE((register_x, register_y)) => {
                    if self.v[register_x as usize] != self.v[register_y as usize] {
                        self.pc += 2;
                    }
                } //if (Vx != Vy) 	Skips the next instruction if VX does not equal VY. (Usually the next instruction is a jump to skip a code block);
                Opcode::LOADI(address) => {
                    self.i = address;
                } //I = NNN 	Sets I to the address NNN.
                Opcode::JUMPI(address) => {
                    self.pc = address + self.v[0] as u16;
                } // PC = V0 + NNN 	Jumps to the address NNN plus V0.
                Opcode::RAND((register, literal)) => {
                    self.v[register as usize] = random::<u8>() & literal;
                } //Vx = rand() & NN 	Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN.
                Opcode::DRAW((register_x, register_y, literal)) => {
                    let x = self.v[register_x as usize] as usize % 64;
                    let y = self.v[register_y as usize] as usize % 32;
                    let height = cmp::min(literal + y as u8, 32) as usize - y;
                    let mut bit_unset = 0;
                    for i in 0..height {
                        let current_row = self.gfx[y + i];
                        let current_wide_row = (current_row as u128) << 64;
                        let sprite_wide_row =
                            (self.memory[self.i as usize + i] as u128) << (128 - 8 - x);
                        self.gfx[y + i] = ((current_wide_row ^ sprite_wide_row) >> 64) as u64;
                        bit_unset |= (current_row ^ self.gfx[y + i]) & current_row;
                    }
                    self.v[0xF] = (bit_unset != 0) as u8;
                } //draw(Vx, Vy, N) 	Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels.
                //  Each row of 8 pixels is read as bit-coded starting from memory location I; I value does not change after the execution of this instruction.
                //  As described above, VF is set to 1 if any screen pixels are flipped from set to unset when the sprite is drawn, and to 0 if that does not happen.
                Opcode::SKPR(register) => {
                    if self.keys & (1 << self.v[register as usize]) != 0 {
                        self.pc += 2;
                    }
                } // if (key() == Vx) 	Skips the next instruction if the key stored in VX is pressed (usually the next instruction is a jump to skip a code block).
                Opcode::SKUP(register) => {
                    if self.keys & (1 << self.v[register as usize]) == 0 {
                        self.pc += 2;
                    }
                } //if (key() != Vx) 	Skips the next instruction if the key stored in VX is not pressed (usually the next instruction is a jump to skip a code block).
                Opcode::MOVED(register) => {
                    self.v[register as usize] = self.delay_timer;
                } //Vx = get_delay() 	Sets VX to the value of the delay timer.
                Opcode::KEYD(register) => {
                    if let Some(key) = self.key_pressed {
                        self.v[register as usize] = key;
                    } else {
                        self.pc -= 2;
                    }
                } //Vx = get_key() 	A key press is awaited, and then stored in VX (blocking operation, all instruction halted until next key event).
                Opcode::LOADD(register) => {
                    self.delay_timer = self.v[register as usize];
                } //delay_timer(Vx) 	Sets the delay timer to VX.
                Opcode::LOADS(register) => {
                    self.sound_timer = self.v[register as usize];
                } //sound_timer(Vx) 	Sets the sound timer to VX.
                Opcode::ADDI(register) => {
                    self.i += self.v[register as usize] as u16;
                } //I += Vx 	Adds VX to I. VF is not affected.[c]
                Opcode::LDSPR(register) => {
                    self.i = FONT_START_ADDRESS + self.v[register as usize] as u16;
                } //I = sprite_addr[Vx] 	Sets I to the location of the sprite for the character in VX. Characters 0-F (in hexadecimal) are represented by a 4x5 font.
                Opcode::BCD(register) => {
                    self.memory[self.i as usize] = self.v[register as usize] / 100;
                    self.memory[(self.i + 1) as usize] = self.v[register as usize] / 10 % 10;
                    self.memory[(self.i + 2) as usize] = (self.v[register as usize] % 100) % 10;
                } // set_BCD(Vx) *(I+0) = BCD(3); *(I+1) = BCD(2); *(I+2) = BCD(1);
                // Stores the binary-coded decimal representation of VX, with the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.
                Opcode::STORE(register) => {
                    self.memory[(self.i as usize)..(self.i + register as u16 + 1) as usize]
                        .copy_from_slice(&self.v[0..(register + 1) as usize]);
                } //reg_dump(Vx, &I) 	Stores from V0 to VX (including VX) in memory, starting at address I. The offset from I is increased by 1 for each value written, but I itself is left unmodified.[d]
                Opcode::READ(register) => self.v[..(register + 1) as usize].copy_from_slice(
                    &self.memory[self.i as usize..(self.i + register as u16 + 1) as usize],
                ), //reg_load(Vx, &I) 	Fills from V0 to VX (including VX) with values from memory, starting at address I. The offset from I is increased by 1 for each value read, but I itself is left unmodified.[d]
            },
            Err(UnknownOpcode) => {
                std::eprintln!("Unknown opcode {:#06X}", self.opcode);
            }
        }

        // execute opcode

        // update timers
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }
}

fn main() {
    let path = std::env::args().nth(1).map(std::path::PathBuf::from);

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "QUIP-8",
        options,
        Box::new(|cc| Box::new(Quip8App::new(cc, path))),
    );
}
