
use rand::Rng;
use std::future::from_generator;

fn main() {
    let asdf: u8 = 255;

    println!("Hello, world!, %d", );
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
    (a & 0b10000000)
}


pub fn get_lowest(a: u8) -> u8 {
    (a & 0b00000001)
}


struct Screen {
    content: vec<vec<bool>>;
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
    generalPurpose: Vec<u8>,
    PC: u16,
    SP: u8,
    DT: u8,
    ST: u8,
    VF: bool,
    rand: rng

}

impl Chip8State {
    pub fn new() -> Chip8State {

    }

    pub fn handle_instruction(&mut self) -> () {
        let cur_high = self.memory[PC];
        let cur_low = self.memory[PC + 1]

        if cur_high == 0{
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
        let lohi = get_highest(cur_lowr);
        match cur_highest {
            1 => self.jump(cur_high, cur_low)
            2 => self.call(cur_low,cur_high)
            3 => self.skipEqual()
            4 => self.skip_not_equal()
            5 => self.skip_vequal()
            6 => self.load_vx()
            7 => self.add()
            8 => match cur_lowest {
                0 => self.load_vx_vy()
                1 => self.or()
                2 => self.and()
                3 => self.xor
                4 => self.add()
                5 => self.sub()
                6 => self.shift_right()
                7 => self.subn()
                14 => self.shift_left()

            }
            9 => self.skip_not_equal()
            10 => self.load_from_memory()
            11 => self.jump(cur_high, cur_low)
            12 => self.random()
            13 => self.load_sprite()
            14 => if true{}
            15 => self.load_key()

        }



    }

    pub fn clear_screen(&mut self) -> () {}

    pub fn ret(&mut self) -> () {
        self.PC = self.SP;
        self.SP = self.SP - 1;
    }

    pub fn sys(a: Chip8State) -> () {}

    pub fn jump(&mut self, l: u8, h: u8) -> () {
        let lo_ = l as u16;
        let hi_ = get_lowest(l) as u16;
        let new_address: u16 = 256 * hi_ + lo_ + self.generalPurpose[0];
        self.PC = new_address;
    }

    pub fn jump_v0(&mut self, l: u8, h: u8) -> () {
        let lo_ = l as u16;
        let hi_ = get_lowest(l) as u16;
        let new_address: u16 = 256 * hi_ + lo_ + self.generalPurpose[0];
        self.PC = new_address;
    }

    pub fn call(&mut self, l: u8, h: u8) -> () {
        let new_address: u8 = 256 * get_lowest(h) + l;
        a.SP = a.SP + 1;
        a.memory[a.SP] = a.PC;
        a.PC = new_address;
    }

    pub fn skipEqual(&mut self, x: u8, kk: u8) -> () {
        if self.generalPurpose[x] == kk {
            self.PC = self.PC + 2;
        }
    }

    pub fn skip_not_equal(&mut self, kk: u8, x: u8) -> () {
        if self.generalPurpose[x] != kk {
            self.PC = self.PC + 2;
        }
    }

    pub fn skip_vequal(&mut self, kk: u8, x: u8, y: u8) -> () {
        if self.generalPurpose[x] == self.generalPurpose[y] {
            self.PC += 2;
        }
    }

    pub fn skip_pressed(&mut self, key: u8) {
        // find keyboard!
    }

    pub fn load_vx(&mut self, x: u8, kk: u8) -> () {}

    pub fn load_key(&mut self, x: u8) {
        stdin().read_line(&mut s).expect("Did not enter a correct string");
    }

    pub fn load_DT(&mut self, x: u8) {
        self.generalPurpose[x] = self.DT;
    }

    pub fn load_ST(&mut self, x: u8) {
        self.generalPurpose[x] = self.ST;
    }

    pub fn load_vx_vy(&mut self, x: u8, y: u8) -> () {
        self.generalPurpose[x] = self.generalPurpose[y];
    }

    pub fn add_immediate(&mut self, x: u8, i: u8) -> () {}

    pub fn add(&mut self, x: u8, y: u8) -> () {
        let a: u8 = self.generalPurpose[x];
        let b: u8 = self.generalPurpose[y];
        if get_highest(a) > 0 && get_highest(b) > 0 {
            self.VF = true;
        }
        self.generalPurpose[x] = self.generalPurpose[x] + self.generalPurpose[y]
    }


    pub fn load_sprite(&mut self) -> () {
        collision : bool = false;
        for x in 1..3{
            for y in 1..8{



            }
        }
    }

    pub fn store_to_memory(&mut self) -> () {}

    pub fn load_from_memory(&mut self, x: u8, y: u8) -> () {
        self.generalPurpose[x] = self.memory[y];
    }

    pub fn load_Rom(&mut self) -> () {
        //start at 0x200, then copy everything into memory
    }

    pub fn random(&mut self, x: u8, kk: u8){
        n : u8 = self.rand.gen();
        self.generalPurpose[x] = n & kk;
    }

    pub fn or(&mut self, x: u8, y:u8){
        self.generalPurpose[x] = self.generalPurpose[x] | self.generalPurpose[y]
    }

    pub fn shift_right(&mut self, x: u8){
        if self.generalPurpose[x] & 0b00000001 > 0{
            self.VF = true;
        }
        self.generalPurpose[x] = self.generalPurpose[x] / 2
    }
    pub fn shift_left(&mut self){
        if self.generalPurpose[x] & 0b10000000 > 0{
            self.VF = true;
        }
        self.generalPurpose[x] = self.generalPurpose[x] * 2
    }
    pub fn subtract(&mut self, x: u8, y:u8){
        if self.generalPurpose[x] > self.generalPurpose[y]{
            self.VF = true;
        }
        self.generalPurpose[x] = self.generalPurpose[x] - self.generalPurpose[y];
    }
    pub fn subtract_not(&mut self, x: u8, y:u8){
        if self.generalPurpose[y] > self.generalPurpose[x]{
            self.VF = true;
        }
        self.generalPurpose[x] = self.generalPurpose[x] - self.generalPurpose[y];

    }

}


struct ROM {
    content: Vec<u8>
}


fn initialize(rom: ROM) {}

