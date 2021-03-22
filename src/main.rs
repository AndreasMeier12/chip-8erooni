use rand::Rng;

use sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels;

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::rect::{Rect, Point};
use std::time::Duration;
use std::path::Path;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, WindowCanvas};
use std::fs;
use std::fs::File;
use std::io::Read;

const PIXEL_EMBIGGENING: u32 = 8;
const CHIP_8_WIDTH: u32 = 64;
const CHIP_8_HEIGHT: u32 = 64;
const SCREEN_WIDTH: u32 = CHIP_8_WIDTH * PIXEL_EMBIGGENING;
const SCREEN_HEIGHT: u32 = CHIP_8_HEIGHT * PIXEL_EMBIGGENING;


pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;


    let path = "testcases/test_opcode.ch8";


    let window = video_subsystem
        .window("rust-sdl2 demo: Video", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut state: Chip8State = Chip8State::new();
    let path: &Path = Path::new("./testcases/test_opcode.ch8");
    let rom = read_in_rom(path);
    state.load_rom(rom);

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;
    let mut accum: u32 = 0;
    let tick_duration: u32 = 1_000_000_000u32 / 500;
    let decrement_rate: u32 = 1_000_000_000u32 / 60;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        draw_screen(state.screen.clone(), &mut canvas);
        canvas.present();
        accum = accum + tick_duration;
        if accum > decrement_rate {
            accum = accum - decrement_rate;
            state.decrement();
        }


        ::std::thread::sleep(Duration::new(0, tick_duration));
        // The rest of the game loop goes here...
    }

    Ok(())
}

fn draw_screen(screen: Vec<bool>, canvas: &mut WindowCanvas) -> () {
    for i in 0..screen.len() - 1 {
        let x = i % CHIP_8_WIDTH as usize;
        let y = i / CHIP_8_WIDTH as usize;
        let x_: i16 = PIXEL_EMBIGGENING as i16 * (x as i16) as i16;
        let y_: i16 = PIXEL_EMBIGGENING as i16 * (y as i16) as i16;
        let asdf: Point = Point::new(x_ as i32, y_ as i32);
        let asdf_rect:Rect = Rect::new(x_ as i32, y_ as i32, PIXEL_EMBIGGENING/2, PIXEL_EMBIGGENING/2);
        if screen[i] && (i % 3 == 0)  {
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas.draw_rect(asdf_rect);
            canvas.fill_rect(asdf_rect);
        } else {
        }


    }
}


const UPPER: u8 = 0b11110000;
const LOWER: u8 = 0b00001111;

pub fn get_lower(a: u8) -> u8 {
    a & LOWER
}

pub fn get_higher(a: u8) -> u8 {
    (a & LOWER) >> 4
}

pub fn get_highest(a: u8) -> u8 {
    a & 0b10000000
}


pub fn get_lowest(a: u8) -> u8 {
    a & 0b00000001
}


pub fn read_in_rom(path: &Path) -> Vec<u8> {
    let mut file = File::open(path).expect("File not found");
    let metadata = fs::metadata(&path).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer).expect("buffer overflow");
    buffer
}


struct Screen {
    content: Vec<Vec<bool>>,
}


enum OpCode {}


struct Register {
    i_upper: u8,
    i_lower: u8,
}


struct Instruction {
    upper: u8,
    lower: u8,

}

struct InstructionType {}

#[derive(Clone)]
struct Chip8State {
    memory: Vec<u8>,
    general_purpose: Vec<u8>,
    pc: u16,
    sp: u8,
    dt: u8,
    st: u8,
    vf: bool,
    i: u16,
    screen: Vec<bool>,
}

impl Chip8State {
    pub fn new() -> Chip8State {
        Chip8State {
            memory: vec![0; 4096],
            general_purpose: vec![0; 16],
            pc: 0,
            sp: 0,
            dt: 0,
            st: 0,
            vf: false,
            i: 0,
            screen: vec![false; 2048],
        }
    }

    pub fn step(&mut self) -> () {
        self.handle_instruction();
        self.pc += 2;
    }

    pub fn decrement(&mut self) -> () {
        if self.dt >= 1 {
            self.dt = self.dt - 1;
        }
        if self.st >= 1 {
            self.st = self.st - 1;
        }
    }

    pub fn handle_instruction(&mut self) -> () {
        let cur_high = self.memory[self.pc as usize];
        let cur_low = self.memory[(self.pc + 1) as usize];

        if cur_high == 0 {
            if cur_low == 0xE0 {
                self.clear_screen();
            }
            if cur_low == 0xEE {
                self.ret();
            }
        }

        let cur_highest = get_highest(cur_high);
        let cur_lowest = get_lowest(cur_low);

        let hilo = get_lowest(cur_high);
        let lohi = get_highest(cur_low);
        match cur_highest {
            1 => self.jump(cur_high, cur_low),
            2 => self.call(cur_low, cur_high),
            3 => self.skip_equal(hilo, cur_low),
            4 => self.skip_not_equal(hilo, cur_lowest),
            5 => self.skip_v_equal(hilo, lohi),
            6 => self.load_vx(hilo, cur_low),
            7 => self.add_byte(hilo, cur_low),
            8 => match cur_lowest {
                0 => self.load_vx_vy(hilo, lohi),
                1 => self.or(hilo, lohi),
                2 => self.and(hilo, lohi),
                3 => self.xor(hilo, lohi),
                4 => self.add(hilo, lohi),
                5 => self.sub(hilo, lohi),
                6 => self.shift_right(hilo),
                7 => self.subn(hilo, lohi),
                14 => self.shift_left(hilo),
                _ => panic!("This shouldn't happen!"),
            },
            9 => self.skip_not_equal(hilo, lohi),
            10 => self.load(hilo, cur_low),
            11 => self.jump(cur_high, cur_low),
            12 => self.random(hilo, cur_low),
            13 => self.load_sprite(hilo, lohi, cur_lowest),
            14 => match cur_lowest {
                1 => self.skip_pressed(hilo),
                14 => self.skip_not_pressed(hilo),
                _ => panic!("Nooooooo!"),
            },
            15 => match cur_low {
                0x07 => self.load_from_dt(hilo),
                0x0a => self.load_key(hilo),
                0x15 => self.load_to_dt(hilo),
                0x18 => self.load_from_st(hilo),
                0x1E => self.add_i(hilo),
                0x29 => self.load_f_vx(hilo),
                0x33 => self.load_b_vx(hilo),
                0x55 => self.load_i_vx(hilo),
                0x65 => self.load_vx_i(hilo),
                _ => println!("This shouldn't happen!, curlow wrong")
            }
            _ => println!("This shouldn't happen!")
        }
    }

    pub fn dump_and_panic(&self) {}

    pub fn dump(&self) {}

    pub fn clear_screen(&mut self) -> () {
        self.screen = vec![false; 2048]
    }

    pub fn ret(&mut self) -> () {
        self.pc = self.memory[self.sp as usize] as u16;
        self.sp = self.sp - 1;
    }


    pub fn jump(&mut self, l: u8, h: u8) -> () {
        let lo_ = l as u16;
        let hi_ = get_lower(h) as u16;
        let new_address: u16 = (256 as u16 * hi_ as u16 + lo_ as u16 + self.general_purpose[0] as u16) as u16;
        self.pc = new_address;
    }

    pub fn jump_v0(&mut self, l: u8, h: u8) -> () {
        let lo_ = l as u16;
        let hi_ = get_lowest(l) as u16;
        let new_address: u16 = (256 * hi_ + lo_ + self.general_purpose[0] as u16) as u16;
        self.pc = new_address;
    }

    pub fn call(&mut self, l: u8, h: u8) -> () {
        let h_ = h & 0x0F;
        let new_address: u16 = 256 * (get_lower(h_) as u16) + l as u16;
        println!("{}", 256 * (get_lower(h_) as u16));
        self.sp = self.sp + 1;
        self.memory[self.sp as usize] = self.pc as u8;
        self.pc = new_address as u16;
    }

    pub fn skip_equal(&mut self, x: u8, kk: u8) -> () {
        if self.general_purpose[x as usize] == kk {
            self.pc = self.pc + 2;
        }
    }

    pub fn skip_not_equal(&mut self, x: u8, kk: u8) -> () {
        if self.general_purpose[x as usize] != kk {
            self.pc = self.pc + 2;
        }
    }

    pub fn skip_v_equal(&mut self, x: u8, y: u8) -> () {
        if self.general_purpose[x as usize] == self.general_purpose[y as usize] {
            self.pc += 2;
        }
    }

    pub fn skip_pressed(&mut self, key: u8) {
        // find keyboard!
    }

    pub fn skip_not_pressed(&mut self, key: u8) {
        // find keyboard!
    }

    pub fn load_vx(&mut self, x: u8, kk: u8) -> () {
        self.general_purpose[x as usize] = kk;
    }

    pub fn load_key(&mut self, x: u8) {
        //stdin().read_line(&mut s).expect("Did not enter a correct string");
    }

    pub fn load_from_dt(&mut self, x: u8) {
        self.general_purpose[x as usize] = self.dt;
    }

    pub fn load_to_dt(&mut self, x: u8) {
        self.dt = self.general_purpose[x as usize];
    }

    pub fn load_from_st(&mut self, x: u8) {
        self.general_purpose[x as usize] = self.st;
    }

    pub fn load_to_st(&mut self, x: u8) {
        self.st = self.general_purpose[x as usize];
    }

    pub fn load_vx_vy(&mut self, x: u8, y: u8) -> () {
        self.general_purpose[x as usize] = self.general_purpose[y as usize];
    }

    pub fn add_i(&mut self, x: u8) -> () {
        self.i = (self.i as u32 + self.general_purpose[x as usize] as u32) as u16;
    }

    pub fn add(&mut self, x: u8, y: u8) -> () {
        let a: u8 = self.general_purpose[x as usize];
        let b: u8 = self.general_purpose[y as usize];
        if get_highest(a) > 0 && get_highest(b) > 0 {
            self.vf = true;
        }
        let x_ = self.general_purpose[x as usize] as i16;
        let y_ = self.general_purpose[y as usize] as i16;

        self.general_purpose[x as usize] = (x_ + y_) as u8;
    }

    pub fn add_byte(&mut self, x: u8, kk: u8) {
        self.general_purpose[x as usize] = self.general_purpose[x as usize] + kk;
    }


    pub fn load_sprite(&mut self, x: u8, y: u8, n: u8) -> () {
        self.vf = false;
        let start_pos = 32 * y as u16 + x as u16;

        for i in 0..8 {
            let cur_pos = start_pos + i;
            //overflow
            let mut comparison_byte: u8 = 0b00000001;
            if x as u16 + i < 64 {
                if self.screen[cur_pos as usize] {
                    self.vf = true;
                }
                self.screen[cur_pos as usize] = (0b1111111 & comparison_byte > 0);
                comparison_byte = comparison_byte >> 1;
            }

            //collision
        }
    }

    pub fn store_to_memory(&mut self, x: u8) -> () {
        for i in 0..x + 1 {
            self.memory[(self.i + i as u16) as usize] = self.general_purpose[i as usize];
        }
    }

    pub fn load_from_memory(&mut self, x: u8) {
        for j in 0..x + 1 {
            let i_ = self.i + j as u16;
            self.general_purpose[j as usize] = self.memory[i_ as usize];
        }
    }

    pub fn load_adress(&mut self, x: u8, y: u8) -> () {
        self.i = (256 * x as u16 + y as u16) as u16;
    }

    pub fn load(&mut self, x: u8, y: u8) -> () {
        let x_: u8 = get_lower(x);
        self.i = (256 * x_ as u16 + y as u16) as u16; //TODO check with load_adress
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) -> () {
        //start at 0x200, then copy everything into memory
        for i in 0x200..(0x200 + rom.len() - 1) {
            self.memory[i] = rom[i - 0x200];
        }
        self.pc = 0x200;
    }

    pub fn random(&mut self, x: u8, kk: u8) {
        let mut rng = rand::thread_rng();
        let n: u8 = rng.gen();
        self.general_purpose[x as usize] = n & kk;
    }

    pub fn or(&mut self, x: u8, y: u8) {
        self.general_purpose[x as usize] = self.general_purpose[x as usize] | self.general_purpose[y as usize]
    }

    pub fn shift_right(&mut self, x: u8) {
        if self.general_purpose[x as usize] & 0b00000001 > 0 {
            self.vf = true;
        } else {
            self.vf = false;
        }
        self.general_purpose[x as usize] = self.general_purpose[x as usize] >> 2;
    }
    pub fn shift_left(&mut self, x: u8) {
        if self.general_purpose[x as usize] & 0b10000000 > 0 {
            self.vf = true;
        } else {
            self.vf = false;
        }
        self.general_purpose[x as usize] = self.general_purpose[x as usize] << 1;
    }
    pub fn load_f_vx(&mut self, x: u8) -> () {
        //load sprite
    }

    pub fn load_b_vx(&mut self, x: u8) -> () {
        let val: u8 = self.general_purpose[x as usize];
        let hundreds = val / 100;
        let tens: u8 = (val % 100) / 10;
        let ones: u8 = (val % 10);
        self.memory[self.i as usize] = hundreds;
        self.memory[(self.i + 1) as usize] = tens;
        self.memory[(self.i + 2) as usize] = ones;
    }

    pub fn and(&mut self, x: u8, y: u8) -> () {
        let x_: usize = x as usize;
        let y_: usize = y as usize;
        self.general_purpose[x_] = self.general_purpose[y_] & self.general_purpose[x_];
    }

    pub fn xor(&mut self, x: u8, y: u8) -> () {
        let x_: usize = x as usize;
        let y_: usize = y as usize;
        self.general_purpose[x_] = self.general_purpose[y_] ^ self.general_purpose[x_];
    }

    pub fn sub(&mut self, x: u8, y: u8) -> () { //TODO
        let x_: usize = x as usize;
        let y_: usize = y as usize;
        if self.general_purpose[x as usize] > self.general_purpose[y as usize] {
            self.vf = true;
        } else { self.vf = false; }
        if (self.general_purpose[y_] >= self.general_purpose[x_]) {
            self.general_purpose[x_] = 0;
        } else {
            self.general_purpose[x_] = self.general_purpose[x_] - self.general_purpose[y_];
        }
    }

    pub fn subn(&mut self, x: u8, y: u8) -> () { //TODO
        let x_: usize = x as usize;
        let y_: usize = y as usize;
        if self.general_purpose[x_] < self.general_purpose[y_] {
            self.vf = true;
        } else { self.vf = false; }

        if (self.general_purpose[y_] >= self.general_purpose[x_]) {
            self.general_purpose[x_] = 0;
        } else {
            self.general_purpose[x_] = self.general_purpose[x_] - self.general_purpose[y_];
        }
    }

    pub fn load_i_vx(&mut self, x: u8) {
        //TODO
        self.i = self.general_purpose[x as usize] as u16;
    }
    pub fn load_vx_i(&mut self, x: u8) {
        //TODO
        self.general_purpose[x as usize] = self.i as u8;
    }


    fn initialize(&mut self, rom: ROM) {
        //for rom content into memory
    }


    pub fn example_screen(&mut self) {
        for i in 0..self.screen.len() - 1 {
            if (i % 2 == 0) {
                self.screen[i as usize] = true;
            }
        }
    }
}


struct ROM {
    content: Vec<u8>
}


#[cfg(test)]
mod tests {
    use crate::Chip8State;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_initial() {
        let a = Chip8State::new();
        assert_eq!(a.i, 0);
    }

    #[test]
    fn test_add_i() {
        let mut a = Chip8State::new();
        assert_eq!(a.general_purpose[5], 0);
        a.i = 13;
        a.general_purpose[5] = 8;
        a.add_i(5);
        assert_eq!(a.i, 21);
    }

    #[test]
    fn test_add() {
        let mut a = Chip8State::new();
        a.general_purpose[3] = 13;
        a.general_purpose[5] = 8;
        a.add(5, 3);
        assert_eq!(a.general_purpose[5], 21);
    }

    #[test]
    fn test_add_overflow() {
        let mut a = Chip8State::new();
        a.general_purpose[3] = 0xFF;
        a.general_purpose[5] = 0xFF;
        a.add(5, 3);
        assert_eq!(a.vf, true);
    }

    #[test]
    fn test_add_kk() {
        let mut a = Chip8State::new();
        a.general_purpose[3] = 13;
        a.general_purpose[5] = 8;
        a.add_byte(5, 18);
        assert_eq!(a.general_purpose[5], 26);
    }

    #[test]
    fn test_call() {
        let mut a = Chip8State::new();
        let stack_pointer: u8 = a.sp;
        let h: u8 = 0xFE;
        let l: u8 = 0xBC;
        a.call(l, h);
        assert_eq!(a.pc, 0x0EBC);
        assert_eq!(a.sp, stack_pointer + 1);
    }

    #[test]
    fn test_jump() {
        let mut a = Chip8State::new();
        let h: u8 = 0xFE;
        let l: u8 = 0xBC;
        a.call(l, h);
        assert_eq!(a.pc, 0x0EBC);
    }


    #[test]
    fn test_ret() {
        let mut a = Chip8State::new();
        a.sp = 10;
        a.memory[10] = 3;
        a.ret();
        assert_eq!(a.pc, 3);
        assert_eq!(a.sp, 9);
    }

    #[test]
    fn test_skip_equal() {
        let mut a = Chip8State::new();
        a.sp = 10;
        a.memory[10] = 3;
        a.pc = 3;
        a.general_purpose[7] = 123;
        a.skip_equal(7, 123);
        assert_eq!(a.pc, 5);
        a.skip_equal(9, 123);
        assert_eq!(a.pc, 5);
    }

    #[test]
    fn test_skip_not_equal() {
        let mut a = Chip8State::new();
        a.sp = 10;
        a.memory[10] = 3;
        a.pc = 3;
        a.general_purpose[7] = 123;
        a.skip_not_equal(7, 123);
        assert_eq!(a.pc, 3);
        a.skip_not_equal(9, 123);
        assert_eq!(a.pc, 5);
    }


    #[test]
    fn test_skip_vequal() {
        let mut a = Chip8State::new();
        a.sp = 10;
        a.memory[10] = 3;
        a.pc = 3;
        a.general_purpose[7] = 123;
        a.general_purpose[6] = 123;
        a.general_purpose[5] = 124;
        a.skip_v_equal(7, 6);
        assert_eq!(a.pc, 5);
        a.skip_v_equal(7, 5);
        assert_eq!(a.pc, 5);
    }

    #[test]
    fn test_load_kk() {
        let mut a = Chip8State::new();
        assert_eq!(a.general_purpose[5], 0);
        a.load_vx(5, 123);
        assert_eq!(a.general_purpose[5], 123);
    }

    #[test]
    fn test_load_vx_vy() {
        let mut a = Chip8State::new();
        assert_eq!(a.general_purpose[5], 0);
        a.general_purpose[3] = 18;
        a.load_vx_vy(5, 3);
        assert_eq!(a.general_purpose[5], 18);
    }

    #[test]
    fn test_or() {
        let mut a = Chip8State::new();
        assert_eq!(a.general_purpose[5], 0);
        a.general_purpose[3] = 0b10010001;
        a.general_purpose[5] = 0b01100101;
        a.or(5, 3);
        assert_eq!(a.general_purpose[5], 0b11110101);
    }

    #[test]
    fn test_xor() {
        let mut a = Chip8State::new();
        assert_eq!(a.general_purpose[5], 0);
        a.general_purpose[3] = 0b10010101;
        a.general_purpose[5] = 0b01100101;
        a.xor(5, 3);
        assert_eq!(a.general_purpose[5], 0b11110000);
    }

    #[test]
    fn test_and() {
        let mut a = Chip8State::new();
        assert_eq!(a.general_purpose[5], 0);
        a.general_purpose[3] = 0b10010101;
        a.general_purpose[5] = 0b11100101;
        a.and(5, 3);
        assert_eq!(a.general_purpose[5], 0b10000101);
    }


    #[test]
    fn test_rand() {
        let mut a = Chip8State::new();
        assert_eq!(a.general_purpose[5], 0);
        a.general_purpose[3] = 13;
        a.general_purpose[5] = 2;
        a.random(5, 0);
        assert_eq!(a.general_purpose[5], 0);
    }

    #[test]
    fn test_shl() {
        let mut a = Chip8State::new();
        assert_eq!(a.general_purpose[3], 0);
        a.general_purpose[3] = 13;
        a.shift_left(3);
        assert_eq!(a.general_purpose[3], 26);
        a.general_purpose[1] = 0b11111111;
        a.shift_left(1);
        assert_eq!(a.vf, true)
    }

    #[test]
    fn test_load_address() {
        let mut a = Chip8State::new();
        assert_eq!(a.general_purpose[3], 0);
        a.i = 0;
        let x: u8 = 0xFA;
        let y: u8 = 0xCB;
        a.load(x, y);
        assert_eq!(a.i, 0x0ACB)
    }

    #[test]
    fn test_set_dt() {
        let mut a = Chip8State::new();
        assert_eq!(a.dt, 0);
        a.general_purpose[8] = 8;
        a.load_to_dt(8);
        assert_eq!(a.dt, 8);
    }

    #[test]
    fn test_load_from_dt() {
        let mut a = Chip8State::new();
        assert_eq!(a.general_purpose[3], 0);
        assert_eq!(a.dt, 0);
        a.general_purpose[8] = 8;
        a.load_to_dt(8);
        assert_eq!(a.dt, 8);
        a.load_from_dt(3);
        assert_eq!(a.general_purpose[3], 8);
    }

    #[test]
    fn test_set_st() {
        let mut a = Chip8State::new();
        a.general_purpose[3] = 12;
        a.load_to_st(3);
        assert_eq!(a.st, 12);
    }

    #[test]
    fn test_load_from_st() {
        let mut a = Chip8State::new();
        a.general_purpose[3] = 12;
        a.load_to_st(3);
        assert_eq!(a.st, 12);
        a.load_from_st(4);
        assert_eq!(a.general_purpose[4], 12);
    }

    #[test]
    fn test_load_memory() {
        let mut a = Chip8State::new();
        a.general_purpose[0] = 1;
        a.general_purpose[1] = 1;
        a.general_purpose[2] = 1;
        a.general_purpose[3] = 1;
        a.memory[257] = 2;
        a.memory[258] = 2;
        a.memory[259] = 2;
        a.i = 257;
        a.load_from_memory(2);
        assert_eq!(a.general_purpose[0], 2);
        assert_eq!(a.general_purpose[1], 2);
        assert_eq!(a.general_purpose[2], 2);
        assert_eq!(a.general_purpose[3], 1);
    }

    #[test]
    fn test_store_to_memory() {
        let mut a = Chip8State::new();
        a.general_purpose[0] = 1;
        a.general_purpose[1] = 1;
        a.general_purpose[2] = 1;
        a.general_purpose[3] = 1;
        a.memory[257] = 2;
        a.memory[258] = 2;
        a.memory[259] = 2;
        a.memory[260] = 2;
        a.i = 257;
        a.store_to_memory(2);
        assert_eq!(a.memory[257], 1);
        assert_eq!(a.memory[258], 1);
        assert_eq!(a.memory[259], 1);
        assert_eq!(a.memory[260], 2);
    }

    #[test]
    fn test_store_binary_to_memory() {
        let mut a = Chip8State::new();
        a.general_purpose[4] = 135;
        a.memory[257] = 2;
        a.memory[258] = 2;
        a.memory[259] = 2;
        a.memory[260] = 2;
        a.i = 257;
        a.load_b_vx(4);
        assert_eq!(a.memory[257], 1);
        assert_eq!(a.memory[258], 3);
        assert_eq!(a.memory[259], 5);
        assert_eq!(a.memory[260], 2);
    }

    #[test]
    fn test_subtract() {
        let mut a = Chip8State::new();
        a.general_purpose[4] = 135;
        a.general_purpose[3] = 12;
        a.general_purpose[5] = 18;
        a.sub(5, 3);
        assert_eq!(a.general_purpose[5], 6);
        assert_eq!(a.vf, true);
        a.sub(3, 4);
        assert_eq!(a.general_purpose[3], 0);
        assert_eq!(a.vf, false)
    }

    #[test]
    fn test_subtract_not() {
        let mut a = Chip8State::new();
        a.general_purpose[4] = 135;
        a.general_purpose[3] = 12;
        a.general_purpose[5] = 18;
        a.subn(5, 3);
        assert_eq!(a.general_purpose[5], 6);
        assert_eq!(a.vf, false);
        a.subn(3, 4);
        assert_eq!(a.general_purpose[3], 0);
        assert_eq!(a.vf, true)
    }
}