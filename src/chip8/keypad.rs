use sdl2::keyboard::Keycode;


const FONTSET_SIZE: usize = 80;
const NUM_KEYS: usize = 16;


const FONTSET: [u8; FONTSET_SIZE] = [
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
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct Keypad {
    keys: [bool; NUM_KEYS],
}


impl Keypad {

    pub fn new() -> Self {
        let k = Self {
            keys: [false; NUM_KEYS]
        };
        k
    }

    pub fn keypress(&mut self, idx: usize, pressed: bool) {
        self.keys[idx] = pressed;
    }

    pub fn get_keys(&self) -> &[bool] {
        &self.keys
    }

    pub fn clear(&mut self) {
        self.keys = [false; NUM_KEYS];
    }

    pub fn get_fontset(&self) -> &[u8] {
        &FONTSET
    }




    /*
    Keyboard                    Chip-8
    +---+---+---+---+           +---+---+---+---+
    | 1 | 2 | 3 | 4 |           | 1 | 2 | 3 | C |
    +---+---+---+---+           +---+---+---+---+
    | Q | W | E | R |           | 4 | 5 | 6 | D |
    +---+---+---+---+     =>    +---+---+---+---+
    | A | S | D | F |           | 7 | 8 | 9 | E |
    +---+---+---+---+           +---+---+---+---+
    | Z | X | C | V |           | A | 0 | B | F |
    +---+---+---+---+           +---+---+---+---+
    */



    pub fn key2btn(&self, key: Keycode) -> Option<usize> {
        match key {
            Keycode::Num1 =>    Some(0x1),
            Keycode::Num2 =>    Some(0x2),
            Keycode::Num3 =>    Some(0x3),
            Keycode::Num4 =>    Some(0xC),
            Keycode::Q =>       Some(0x4),
            Keycode::W =>       Some(0x5),
            Keycode::E =>       Some(0x6),
            Keycode::R =>       Some(0xD),
            Keycode::A =>       Some(0x7),
            Keycode::S =>       Some(0x8),
            Keycode::D =>       Some(0x9),
            Keycode::F =>       Some(0xE),
            Keycode::Z =>       Some(0xA),
            Keycode::X =>       Some(0x0),
            Keycode::C =>       Some(0xB),
            Keycode::V =>       Some(0xF),
            _ =>                None,
        }
    }
}

