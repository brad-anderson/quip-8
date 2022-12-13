#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use rand::prelude::*;

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

type Address = u16;
type RegisterAddress = u8;
type Literal = u8;

enum Opcode {
    // 0x0NNN - Call
    // Calls machine code routine (RCA 1802 for COSMAC VIP) at address NNN. Not necessary for most ROMs.
    CallMachineCode(Address),

    // 0x00E0 - Display
    // C Pseudo: disp_clear()
    // Clears the screen.
    ClearDisplay,

    // 0x00EE - Flow
    // C Pseudo: return;
    // Returns from a subroutine.
    Return,

    // 0x1NNN - Flow
    // C Pseudo: goto NNN;
    // Jumps to address NNN.
    Jump(Address),

    // 0x2NNN - Flow
    // C Pseudo: *(0xNNN)()
    // Calls subroutine at NNN.
    Call(Address),

    // 0x3XNN - Cond
    // C Pseudo: if (Vx == NN)
    // Skips the next instruction if VX equals NN (usually the next instruction
    // is a jump to skip a code block).
    IfLiteralEqual((RegisterAddress, Literal)),

    // 0x4XNN - Cond
    // C Pseudo: if (Vx != NN)
    // Skips the next instruction if VX does not equal NN (usually the next
    // instruction is a jump to skip a code block).
    IfLiteralNotEqual((RegisterAddress, Literal)),

    // 0x5XY0 - Cond
    // C Pseudo: if (Vx == Vy)
    // Skips the next instruction if VX equals VY (usually the next instruction
    // is a jump to skip a code block).
    IfEqual((RegisterAddress, RegisterAddress)),

    // 0x6XNN - Const
    // C Pseudo: Vx = NN
    // Sets VX to NN.
    LoadLiteral((RegisterAddress, Literal)),

    // 0x7XNN - Const
    // C Pseudo: Vx += NN
    // Adds NN to VX (carry flag is not changed).
    AddLiteral((RegisterAddress, Literal)),

    // 0x8XY0 - Assig
    // C Pseudo: Vx = Vy
    // Sets VX to the value of VY.
    Copy((RegisterAddress, RegisterAddress)),

    // 0x8XY1 - BitOp
    // C Pseudo: Vx |= Vy
    // Sets VX to VX or VY. (bitwise OR operation)
    Or((RegisterAddress, RegisterAddress)),

    // 0x8XY2 - BitOp
    // C Pseudo: Vx &= Vy
    // Sets VX to VX and VY. (bitwise AND operation)
    And((RegisterAddress, RegisterAddress)),

    // 0x8XY3[a] - BitOp
    // C Pseudo: Vx ^= Vy
    // Sets VX to VX xor VY.
    Xor((RegisterAddress, RegisterAddress)),

    // 0x8XY4 - Math
    // C Pseudo: Vx += Vy
    // Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there
    // is not.
    Add((RegisterAddress, RegisterAddress)),

    // 0x8XY5 - Math
    // C Pseudo: Vx -= Vy
    // VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1
    // when there is not.
    Subtract((RegisterAddress, RegisterAddress)),

    // 0x8XY6 - BitOp
    // C Pseudo: Vx >>= 1
    // Stores the least significant bit of VX in VF and then shifts VX to the
    // right by 1.
    // Better Name?
    BitshiftRightOne(RegisterAddress),

    // 0x8XY7 - Math
    // C Pseudo: Vx = Vy - Vx
    // Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when
    //  there is not.
    // Better Name?
    RevSub((RegisterAddress, RegisterAddress)),

    // 0x8XYE - BitOp
    // C Pseudo: Vx <<= 1
    // Stores the most significant bit of VX in VF and then shifts VX to the
    // left by 1.
    // Better Name?
    BitshiftLeftOne(RegisterAddress),

    // 0x9XY0 - Cond
    // C Pseudo: if (Vx != Vy)
    // Skips the next instruction if VX does not equal VY. (Usually the next
    // instruction is a jump to skip a code block);
    IfNotEqual((RegisterAddress, RegisterAddress)),

    // 0xANNN - MEM
    // C Pseudo: I = NNN
    // Sets I to the address NNN.
    LoadI(Address),

    // 0xBNNN - Flow
    // C Pseudo: PC = V0 + NNN
    // Jumps to the address NNN plus V0.
    OffsetJump(Address),

    // 0xCXNN - Rand
    // C Pseudo: Vx = rand() & NN
    // Sets VX to the result of a bitwise and operation on a random number
    // (Typically: 0 to 255) and NN.
    LoadRandom((RegisterAddress, Literal)),

    // 0xDXYN - Display
    // C Pseudo: draw(Vx, Vy, N)
    // Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and
    // a height of N pixels. Each row of 8 pixels is read as bit-coded starting
    // from memory location I; I value does not change after the execution of
    // this instruction. As described above, VF is set to 1 if any screen pixels
    // are flipped from set to unset when the sprite is drawn, and to 0 if that
    // does not happen.
    DrawSprite((RegisterAddress, RegisterAddress, Literal)),

    // 0xEX9E - KeyOp
    // C Pseudo: if (key() == Vx)
    // Skips the next instruction if the key stored in VX is pressed (usually
    // the next instruction is a jump to skip a code block).
    IfKey(RegisterAddress),

    // 0xEXA1 - KeyOp
    // C Pseudo: if (key() != Vx)
    // Skips the next instruction if the key stored in VX is not pressed
    // (usually the next instruction is a jump to skip a code block).
    IfNotKey(RegisterAddress),

    // 0xFX07 - Timer
    // C Pseudo: Vx = get_delay()
    // Sets VX to the value of the delay timer.
    LoadFromDelayTimer(RegisterAddress),

    // 0xFX0A - KeyOp
    // C Pseudo: Vx = get_key()
    // A key press is awaited, and then stored in VX (blocking operation, all
    // instruction halted until next key event).
    AwaitKey(RegisterAddress),

    // 0xFX15 - Timer
    // C Pseudo: delay_timer(Vx)
    // Sets the delay timer to VX.
    LoadDelayTimer(RegisterAddress),

    // 0xFX18 - Sound
    // C Pseudo: sound_timer(Vx)
    // Sets the sound timer to VX.
    LoadSoundTimer(RegisterAddress),

    // 0xFX1E - MEM
    // C Pseudo: I += Vx
    // Adds VX to I. VF is not affected.[c]
    AddI(RegisterAddress),

    // 0xFX29 - MEM
    // C Pseudo: I = sprite_addr[Vx]
    // Sets I to the location of the sprite for the character in VX. Characters
    //  0-F (in hexadecimal) are represented by a 4x5 font.
    LoadSpriteAddress(RegisterAddress),

    // 0xFX33 - BCD
    // C Pseudo: set_BCD(Vx) *(I+0) = BCD(3); *(I+1) = BCD(2); *(I+2) = BCD(1);
    // Stores the binary-coded decimal representation of VX, with the hundreds
    // digit in memory at location in I, the tens digit at location I+1, and
    // the ones digit at location I+2.
    BinaryCodedDecimal(RegisterAddress),

    // 0xFX55 - MEM
    // C Pseudo: reg_dump(Vx, &I)
    // Stores from V0 to VX (including VX) in memory, starting at address I.
    // The offset from I is increased by 1 for each value written, but I itself
    // is left unmodified.[d]
    StoreRegisters(RegisterAddress),

    // 0xFX65 - MEM
    // C Pseudo: reg_load(Vx, &I)
    // Fills from V0 to VX (including VX) with values from memory, starting at
    // address I. The offset from I is increased by 1 for each value read, but
    // I itself is left unmodified.[d]
    LoadRegisters(RegisterAddress),
}

struct UnknownOpcode;

impl Opcode {
    pub fn decode(raw_opcode: u16) -> Result<Opcode, UnknownOpcode> {
        match raw_opcode & 0xF000 {
            0x0000 => match raw_opcode {
                0x00E0 => Ok(Opcode::ClearDisplay),
                0x00EE => Ok(Opcode::Return),
                _ => Ok(Opcode::CallMachineCode((raw_opcode & 0x0FFF) as Address)),
            },
            0x1000 => Ok(Opcode::Jump((raw_opcode & 0x0FFF) as Address)),
            0x2000 => Ok(Opcode::Call((raw_opcode & 0x0FFF) as Address)),
            0x3000 => Ok(Opcode::IfLiteralEqual((
                ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                (raw_opcode & 0x00FF) as Literal,
            ))),
            0x4000 => Ok(Opcode::IfLiteralNotEqual((
                ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                (raw_opcode & 0x00FF) as Literal,
            ))),
            0x5000 => Ok(Opcode::IfEqual((
                ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                ((raw_opcode & 0x00F0) >> 6) as RegisterAddress,
            ))),
            0x6000 => Ok(Opcode::LoadLiteral((
                ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                (raw_opcode & 0x00FF) as Literal,
            ))),
            0x7000 => Ok(Opcode::AddLiteral((
                ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                (raw_opcode & 0x00FF) as Literal,
            ))),
            0x8000 => match raw_opcode & 0x000F {
                0x0000 => Ok(Opcode::Copy((
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                    ((raw_opcode & 0x00F0) >> 6) as RegisterAddress,
                ))),
                0x0001 => Ok(Opcode::Or((
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                    ((raw_opcode & 0x00F0) >> 6) as RegisterAddress,
                ))),
                0x0002 => Ok(Opcode::And((
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                    ((raw_opcode & 0x00F0) >> 6) as RegisterAddress,
                ))),
                0x0003 => Ok(Opcode::Xor((
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                    ((raw_opcode & 0x00F0) >> 6) as RegisterAddress,
                ))),
                0x0004 => Ok(Opcode::Add((
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                    ((raw_opcode & 0x00F0) >> 6) as RegisterAddress,
                ))),
                0x0005 => Ok(Opcode::Subtract((
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                    ((raw_opcode & 0x00F0) >> 6) as RegisterAddress,
                ))),
                0x0006 => Ok(Opcode::BitshiftRightOne(
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                )),
                0x0007 => Ok(Opcode::RevSub((
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                    ((raw_opcode & 0x00F0) >> 6) as RegisterAddress,
                ))),
                0x0008 => Ok(Opcode::BitshiftLeftOne(
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                )),
                _ => Err(UnknownOpcode),
            },
            0x9000 => Ok(Opcode::IfNotEqual((
                ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                ((raw_opcode & 0x00F0) >> 6) as RegisterAddress,
            ))),
            0xA000 => Ok(Opcode::LoadI((raw_opcode & 0x0FFF) as Address)),
            0xB000 => Ok(Opcode::OffsetJump((raw_opcode & 0x0FFF) as Address)),
            0xC000 => Ok(Opcode::LoadRandom((
                ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                (raw_opcode & 0x00FF) as Literal,
            ))),
            0xD000 => Ok(Opcode::DrawSprite((
                ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                ((raw_opcode & 0x00F0) >> 6) as RegisterAddress,
                (raw_opcode & 0x000F) as Literal,
            ))),
            0xE000 => match raw_opcode & 0x00FF {
                0x009E => Ok(Opcode::IfKey(
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                )),
                0x00A1 => Ok(Opcode::IfNotKey(
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                )),
                _ => Err(UnknownOpcode),
            },
            0xF000 => match raw_opcode & 0x00FF {
                0x0007 => Ok(Opcode::LoadFromDelayTimer(
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                )),
                0x000A => Ok(Opcode::AwaitKey(
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                )),
                0x0015 => Ok(Opcode::LoadDelayTimer(
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                )),
                0x0018 => Ok(Opcode::LoadSoundTimer(
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                )),
                0x001E => Ok(Opcode::AddI(
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                )),
                0x0029 => Ok(Opcode::LoadSpriteAddress(
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                )),
                0x0033 => Ok(Opcode::BinaryCodedDecimal(
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                )),
                0x0055 => Ok(Opcode::StoreRegisters(
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                )),
                0x0065 => Ok(Opcode::LoadRegisters(
                    ((raw_opcode & 0x0F00) >> 8) as RegisterAddress,
                )),
                _ => Err(UnknownOpcode),
            },

            _ => Err(UnknownOpcode),
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
            s.memory[512..].copy_from_slice(file_contents.as_slice());
        }
        s.next_opcode = (s.memory[s.pc as usize] as u16) << 8 | s.memory[s.pc as usize + 1] as u16;
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
                Opcode::CallMachineCode(_address) => {
                    std::eprintln!("Unimplemented opcode {:#06x}", self.opcode);
                } //Calls machine code routine (RCA 1802 for COSMAC VIP) at address NNN. Not necessary for most ROMs.
                Opcode::ClearDisplay => {
                    std::eprintln!("Unimplemented opcode {:#06x}", self.opcode);
                } // 	disp_clear() 	Clears the screen.
                Opcode::Return => {
                    std::eprintln!("Unimplemented opcode {:#06x}", self.opcode);
                } //return; 	Returns from a subroutine.
                Opcode::Jump(address) => {
                    self.pc = address;
                } //goto NNN; 	Jumps to address NNN.
                Opcode::Call(address) => {
                    std::eprintln!("Unimplemented opcode {:#06x}", self.opcode);
                } //*(0xNNN)() 	Calls subroutine at NNN.
                Opcode::IfLiteralEqual((register, literal)) => {
                    if self.v[register as usize] == literal {
                        self.pc += 2;
                    }
                } //if (Vx == NN) 	Skips the next instruction if VX equals NN (usually the next instruction is a jump to skip a code block).
                Opcode::IfLiteralNotEqual((register, literal)) => {
                    if self.v[register as usize] != literal {
                        self.pc += 2;
                    }
                } //if (Vx != NN) 	Skips the next instruction if VX does not equal NN (usually the next instruction is a jump to skip a code block).
                Opcode::IfEqual((register_x, register_y)) => {
                    if self.v[register_x as usize] == self.v[register_y as usize] {
                        self.pc += 2;
                    }
                } //if (Vx == Vy) 	Skips the next instruction if VX equals VY (usually the next instruction is a jump to skip a code block).
                Opcode::LoadLiteral((register, literal)) => {
                    self.v[register as usize] = literal;
                }
                Opcode::AddLiteral((register, literal)) => {
                    self.v[register as usize] += literal;
                } //Vx += NN 	Adds NN to VX (carry flag is not changed).
                Opcode::Copy((register_x, register_y)) => {
                    self.v[register_x as usize] = self.v[register_y as usize];
                } //Vx = Vy 	Sets VX to the value of VY.
                Opcode::Or((register_x, register_y)) => {
                    self.v[register_x as usize] |= self.v[register_y as usize];
                } // Vx |= Vy 	Sets VX to VX or VY. (bitwise OR operation)
                Opcode::And((register_x, register_y)) => {
                    self.v[register_x as usize] &= self.v[register_y as usize];
                } //Vx &= Vy 	Sets VX to VX and VY. (bitwise AND operation)
                Opcode::Xor((register_x, register_y)) => {
                    self.v[register_x as usize] ^= self.v[register_y as usize];
                } // Vx ^= Vy 	Sets VX to VX xor VY.
                Opcode::Add((register_x, register_y)) => {
                    self.v[register_x as usize] += self.v[register_y as usize];
                } // Vx += Vy 	Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there is not.
                Opcode::Subtract((register_x, register_y)) => {
                    self.v[register_x as usize] -= self.v[register_y as usize];
                } // Vx -= Vy 	VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there is not.
                Opcode::BitshiftRightOne(register) => {
                    self.v[0xf] = self.v[register as usize] & 1;
                    self.v[register as usize] >>= 1;
                } // Vx >>= 1 	Stores the least significant bit of VX in VF and then shifts VX to the right by 1.[b]
                Opcode::RevSub((register_x, register_y)) => {
                    self.v[register_x as usize] =
                        self.v[register_y as usize] - self.v[register_x as usize];
                } // Vx = Vy - Vx 	Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there is not.
                Opcode::BitshiftLeftOne(register) => {
                    self.v[0xf] = self.v[register as usize] & 0xF0;
                    self.v[register as usize] <<= 1;
                } // Vx <<= 1 	Stores the most significant bit of VX in VF and then shifts VX to the left by 1.[b]
                Opcode::IfNotEqual((register_x, register_y)) => {
                    if self.v[register_x as usize] != self.v[register_y as usize] {
                        self.pc += 2;
                    }
                } //if (Vx != Vy) 	Skips the next instruction if VX does not equal VY. (Usually the next instruction is a jump to skip a code block);
                Opcode::LoadI(address) => {
                    self.i = address;
                } //I = NNN 	Sets I to the address NNN.
                Opcode::OffsetJump(address) => {
                    self.pc = address + self.v[0] as u16;
                } // PC = V0 + NNN 	Jumps to the address NNN plus V0.
                Opcode::LoadRandom((register, literal)) => {
                    self.v[register as usize] = random::<u8>() & literal;
                } //Vx = rand() & NN 	Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN.
                Opcode::DrawSprite((register_x, register_y, literal)) => {
                    std::eprintln!("Unimplemented opcode {:#06x}", self.opcode);
                } //draw(Vx, Vy, N) 	Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels. Each row of 8 pixels is read as bit-coded starting from memory location I; I value does not change after the execution of this instruction. As described above, VF is set to 1 if any screen pixels are flipped from set to unset when the sprite is drawn, and to 0 if that does not happen.
                Opcode::IfKey(register) => {
                    std::eprintln!("Unimplemented opcode {:#06x}", self.opcode);
                } // if (key() == Vx) 	Skips the next instruction if the key stored in VX is pressed (usually the next instruction is a jump to skip a code block).
                Opcode::IfNotKey(register) => {
                    std::eprintln!("Unimplemented opcode {:#06x}", self.opcode);
                } //if (key() != Vx) 	Skips the next instruction if the key stored in VX is not pressed (usually the next instruction is a jump to skip a code block).
                Opcode::LoadFromDelayTimer(register) => {
                    self.v[register as usize] = self.delay_timer;
                } //Vx = get_delay() 	Sets VX to the value of the delay timer.
                Opcode::AwaitKey(register) => {
                    std::eprintln!("Unimplemented opcode {:#06x}", self.opcode);
                } //Vx = get_key() 	A key press is awaited, and then stored in VX (blocking operation, all instruction halted until next key event).
                Opcode::LoadDelayTimer(register) => {
                    self.delay_timer = self.v[register as usize];
                } //delay_timer(Vx) 	Sets the delay timer to VX.
                Opcode::LoadSoundTimer(register) => {
                    self.sound_timer = self.v[register as usize];
                } //sound_timer(Vx) 	Sets the sound timer to VX.
                Opcode::AddI(register) => {
                    self.i += self.v[register as usize] as u16;
                } //I += Vx 	Adds VX to I. VF is not affected.[c]
                Opcode::LoadSpriteAddress(register) => {
                    std::eprintln!("Unimplemented opcode {:#06x}", self.opcode);
                } //I = sprite_addr[Vx] 	Sets I to the location of the sprite for the character in VX. Characters 0-F (in hexadecimal) are represented by a 4x5 font.
                Opcode::BinaryCodedDecimal(register) => {
                    std::eprintln!("Unimplemented opcode {:#06x}", self.opcode);
                } // set_BCD(Vx) *(I+0) = BCD(3); *(I+1) = BCD(2); *(I+2) = BCD(1);  Stores the binary-coded decimal representation of VX, with the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.
                Opcode::StoreRegisters(register) => {
                    self.memory[(self.i as usize)..(register + 1) as usize]
                        .copy_from_slice(self.v.as_slice());
                } //reg_dump(Vx, &I) 	Stores from V0 to VX (including VX) in memory, starting at address I. The offset from I is increased by 1 for each value written, but I itself is left unmodified.[d]
                Opcode::LoadRegisters(register) => self.v[..(register + 1) as usize]
                    .copy_from_slice(&self.memory[self.i as usize..]), //reg_load(Vx, &I) 	Fills from V0 to VX (including VX) with values from memory, starting at address I. The offset from I is increased by 1 for each value read, but I itself is left unmodified.[d]
            },
            Err(UnknownOpcode) => {
                std::eprintln!("Unknown opcode {:#06x}", self.opcode);
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
