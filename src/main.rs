
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
    a & 0b10000000
}


pub fn get_lowest(a: u8) -> u8 {
    a & 0b00000001
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
    generalPurpose: Vec<u8>,
    PC: u16,
    SP: u8,
    DT: u8,
    ST: u8,
    VF: bool,
    I: u16,

}

impl Chip8State {
    pub fn new() -> Chip8State {
        Chip8State{
            memory: vec![0; 4096],
            generalPurpose:vec![0; 16],
            PC: 0,
            SP: 0,
            DT: 0,
            ST: 0,
            VF: false,
            I: 0,
        }

    }

    pub fn handle_instruction(&mut self) -> () {
        let cur_high = self.memory[self.PC as usize];
        let cur_low = self.memory[(self.PC + 1) as usize];

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
        let lohi = get_highest(cur_low);
        match cur_highest {
            1 => self.jump(cur_high, cur_low),
            2 => self.call(cur_low,cur_high),
            3 => self.skipEqual(hilo, cur_low),
            4 => self.skip_not_equal(hilo, cur_lowest),
            5 => self.skip_vequal(hilo, lohi),
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
            14 => match  cur_lowest{
                1 => self.skip_pressed(hilo),
                14 => self.skip_not_pressed(hilo),
                _ => panic!("Nooooooo!"),

            },
            15 => match cur_low {
                0x07 => self.load_from_DT(hilo),
                0x0a => self.load_key(hilo),
                0x15 => self.load_to_DT(hilo),
                0x18 => self.load_ST(hilo),
                0x1E => self.add_I(hilo),
                0x29 => self.load_f_vx(hilo),
                0x33 => self.load_b_vx(hilo),
                0x55 => self.load_i_vx(hilo),
                0x65 => self.load_vx_i(hilo),
                _ => println!("This shouldn't happen!, curlow wrong")

            }
            _ => println!("This shouldn't happen!")

        }



    }

    pub fn dump_and_panic(&self){

    }

    pub fn dump(&self){

    }

    pub fn clear_screen(&mut self) -> () {}

    pub fn ret(&mut self) -> () {
        self.PC = self.SP as u16;
        self.SP = self.SP - 1;
    }

    pub fn sys(a: Chip8State) -> () {}

    pub fn jump(&mut self, l: u8, h: u8) -> () {
        let lo_ = l as u16;
        let hi_ = get_lowest(l) as u16;
        let new_address: u16 = (256 as u16 * hi_ as u16 + lo_ as u16 + self.generalPurpose[0] as u16) as u16;
        self.PC = new_address;
    }

    pub fn jump_v0(&mut self, l: u8, h: u8) -> () {
        let lo_ = l as u16;
        let hi_ = get_lowest(l) as u16;
        let new_address: u16 = (256 * hi_ + lo_ + self.generalPurpose[0] as u16) as u16;
        self.PC = new_address;
    }

    pub fn call(&mut self, l: u8, h: u8) -> () {
        let new_address: u16 = 256 * (get_lowest(h)as u16) + l as u16;
        self.SP = self.SP + 1;
        self.memory[self.SP as usize] = self.PC as u8;
        self.PC = new_address as u16;
    }

    pub fn skipEqual(&mut self, x: u8, kk: u8) -> () {
        if self.generalPurpose[x as usize] == kk {
            self.PC = self.PC + 2;
        }
    }

    pub fn skip_not_equal(&mut self, kk: u8, x: u8) -> () {
        if self.generalPurpose[x as usize] != kk {
            self.PC = self.PC + 2;
        }
    }

    pub fn skip_vequal(&mut self, x: u8, y: u8) -> () {
        if self.generalPurpose[x as usize] == self.generalPurpose[y as usize] {
            self.PC += 2;
        }
    }

    pub fn skip_pressed(&mut self, key: u8) {
        // find keyboard!
    }

    pub fn skip_not_pressed(&mut self, key: u8) {
        // find keyboard!
    }

    pub fn load_vx(&mut self, x: u8, kk: u8) -> () {}

    pub fn load_key(&mut self, x: u8) {
        //stdin().read_line(&mut s).expect("Did not enter a correct string");
    }

    pub fn load_from_DT(&mut self, x: u8) {
        self.generalPurpose[x as usize] = self.DT;
    }

    pub fn load_to_DT(&mut self, x: u8){
        self.DT = self.generalPurpose[x as usize];
    }

    pub fn load_ST(&mut self, x: u8) {
        self.generalPurpose[x as usize] = self.ST;
    }

    pub fn load_vx_vy(&mut self, x: u8, y: u8) -> () {
        self.generalPurpose[x as usize] = self.generalPurpose[y as usize];
    }

    pub fn add_I(&mut self, x: u8) -> () {
        self.I = self.I + self.generalPurpose[x as usize] as u16;
    }

    pub fn add(&mut self, x: u8, y: u8) -> () {
        let a: u8 = self.generalPurpose[x as usize];
        let b: u8 = self.generalPurpose[y as usize];
        if get_highest(a) > 0 && get_highest(b) > 0 {
            self.VF = true;
        }
        self.generalPurpose[x as usize] = self.generalPurpose[x as usize] + self.generalPurpose[y as usize]
    }

    pub fn add_byte(&mut self, x: u8, kk:u8){
        self.generalPurpose[x as usize] = self.generalPurpose[x as usize] + kk;
    }


    pub fn load_sprite(&mut self, x: u8, y:u8, n: u8) -> () {
        let collision : bool = false;
        for x in 1..3{
            for y in 1..8{




            }
        }
    }

    pub fn store_to_memory(&mut self, x: u8) -> () {
        for i in 0..self.generalPurpose[x as usize]{
            self.memory[(self.I + i as u16) as usize] = self.generalPurpose[i as usize];
        }
    }

    pub fn load_from_memory(&mut self, x: u8){
        for i in 0..self.generalPurpose[x as usize]{
            let i_ =(self.I + i as u16);
            self.generalPurpose[i as usize] =  self.memory[i_ as usize];
        }
    }

    pub fn load_adress(&mut self, x:u8, y:u8) -> () {
        self.I = (256 * x as u16 + y as u16) as u16;
    }

    pub fn load(&mut self, x: u8, y: u8) -> () {
        self.I = (256 * x as u16 + y as u16) as u16; //TODO check with load_adress
    }

    pub fn load_Rom(&mut self) -> () {
        //start at 0x200, then copy everything into memory
    }

    pub fn random(&mut self, x: u8, kk: u8){
        let n:u8 = 12; //TODO random
        self.generalPurpose[x as usize] = n & kk;
    }

    pub fn or(&mut self, x: u8, y:u8){
        self.generalPurpose[x as usize] = self.generalPurpose[x as usize] | self.generalPurpose[y as usize]
    }

    pub fn shift_right(&mut self, x: u8){
        if self.generalPurpose[x as usize] & 0b00000001 > 0{
            self.VF = true;
        }
        self.generalPurpose[x as usize] = self.generalPurpose[x as usize] / 2
    }
    pub fn shift_left(&mut self, x: u8){
        if self.generalPurpose[x as usize] & 0b10000000 > 0{
            self.VF = true;
        }
        self.generalPurpose[x as usize] = self.generalPurpose[x as usize] * 2
    }
    pub fn subtract(&mut self, x: u8, y:u8){
        if self.generalPurpose[x as usize] > self.generalPurpose[y as usize]{
            self.VF = true;
        }
        self.generalPurpose[x as usize] = self.generalPurpose[x as usize] - self.generalPurpose[y as usize];
    }
    pub fn subtract_not(&mut self, x: u8, y:u8){
        if self.generalPurpose[y as usize] > self.generalPurpose[x as usize]{
            self.VF = true;
        }
        self.generalPurpose[x as usize] = self.generalPurpose[x as usize] - self.generalPurpose[y as usize];

    }
    pub fn load_f_vx(&mut self, x: u8) -> (){
        //load sprite
    }

    pub fn load_b_vx(&mut self, x: u8)-> (){
        //load sprite
    }

    pub fn and(&mut self, x: u8, y:u8) -> (){
        let x_: usize = x as usize;
        let y_: usize = y as usize;
        self.generalPurpose[x_] = self.generalPurpose[y_] & self.generalPurpose[x_];

    }

    pub fn xor(&mut self, x: u8, y:u8) -> (){
        let x_: usize = x as usize;
        let y_: usize = y as usize;
        self.generalPurpose[x_] = self.generalPurpose[y_] ^ self.generalPurpose[x_];

    }

    pub fn sub(&mut self, x: u8, y:u8) -> (){ //TODO
        let x_: usize = x as usize;
        let y_: usize = y as usize;
        self.generalPurpose[x_] = self.generalPurpose[y_] ^ self.generalPurpose[x_];

    }

    pub fn subn(&mut self, x: u8, y:u8) -> (){ //TODO
        let x_: usize = x as usize;
        let y_: usize = y as usize;
        self.generalPurpose[x_] = self.generalPurpose[y_] ^ self.generalPurpose[x_];

    }

    pub fn load_i_vx(&mut self, x: u8){
        //TODO
        self.I = self.generalPurpose[x as usize] as u16;
    }
    pub fn load_vx_i(&mut self, x: u8){
        //TODO
        self.generalPurpose[x as usize] = self.I as u8;
    }


    fn initialize(&mut self, rom: ROM) {
        //for rom content into memory
    }




}


struct ROM {
    content: Vec<u8>
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }




}