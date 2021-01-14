fn main() {
    let asdf: u8 = 255;

    println!("Hello, world!, %d",);
}

const UPPER: u8 = 0b11110000;
const LOWER: u8 = 0b00001111;

pub fn get_lower(a: u8) -> u8 {
    a & LOWER

}

pub fn get_higher(a:u8) -> u8 {
    (a & LOWER) >> 4

}

pub fn get_highest(a:u8) -> u8 {
    (a & 0b10000000)
}


pub fn get_lowest(a:u8) -> u8 {
    (a & 0b00000001)
}



struct Screen {}


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

}

impl Chip8State {
    pub fn handle_instruction(&mut self) -> () {}

    pub fn clear_screen(&mut self) -> () {}

    pub fn ret(&mut self) -> () {
        self.PC = self.SP;
        self.SP = self.SP - 1;
    }

    pub fn sys(a: Chip8State) -> () {}

    pub fn jump(&mut self, l: u8, h: u8) -> () {
        let lo_ = l as u16;
        let hi_ = h as u16;
        let new_address: u16 = 256 * hi_ + lo_;
        self.PC = new_address;
    }

    pub fn call(&mut self, l: u8, h: u8) -> () {
        let new_address: u8 = 256 * h + l;
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

    pub fn load_vx(&mut self, x: u8, kk: u8) -> () {

    }

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

    pub fn add_immediate(&mut self, x: u8, i: u8) -> () {

    }

    pub fn add(&mut self, x: u8, y:u8) -> (){
        let a: u8 = self.generalPurpose[x];
        let b: u8 = self.generalPurpose[y];
        if get_highest(a) > 0 && get_highest(b) > 0 {
            self.VF = true;
        }
        self.generalPurpose[x] = self.generalPurpose[x] + self.generalPurpose[y]
    }


    pub fn load_sprite(&mut self) -> () {}

    pub fn store_to_memory(&mut self) -> () {}

    pub fn load_from_memory(&mut self) -> () {}

    pub fn load_Rom(&mut self) -> () {
        //start at 0x200, then copy everything into memory
    }
}

struct ROM {
    content: Vec<u8>
}


fn initialize(rom: ROM) {}

