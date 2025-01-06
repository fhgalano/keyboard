pub const CUSTOM_KEY: u8 = 0xFF;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Key {
    // None
    NOKEY,
    // modifiers 
    LCTRL,
    RCTRL,
    LSHIFT,
    RSHIFT,
    LALT,
    RALT,
    LSUPER,
    RSUPER,
    // custom modifiers
    HAT,
    // normal keys
    Dd,
    Ee,
    Zz,
}

impl Key {
    pub const fn scancode(&self) -> u8 {
        match *self {
            Self::NOKEY => 0x00,

            Self::LCTRL => 0xE0,
            Self::RCTRL => 0xE4,
            Self::LSHIFT => 0xE1,
            Self::RSHIFT => 0xE5,
            Self::LALT => 0xE2,
            Self::RALT => 0xE6,
            Self::LSUPER => 0xE3,
            Self::RSUPER => 0xE7,

            Self::HAT => CUSTOM_KEY,

            Self::Dd => 0x07,
            Self::Ee => 0x08,
            Self::Zz => 0x1D,
        }
    }

    pub fn modifier_scancodes() -> [u8; 9] {
        [
            Self::LCTRL.scancode(),
            Self::LSHIFT.scancode(), 
            Self::LALT.scancode(),
            Self::LSUPER.scancode(),
            Self::RCTRL.scancode(),
            Self::RSHIFT.scancode(),
            Self::RALT.scancode(),
            Self::RSUPER.scancode(),
            // custom modifiers
            Self::HAT.scancode(),
        ]
    }

    pub fn modifiers() -> [Self; 9] {
        [
            Self::LCTRL,
            Self::LSHIFT,
            Self::LALT,
            Self::LSUPER,
            Self::RCTRL,
            Self::RSHIFT,
            Self::RALT,
            Self::RSUPER,
            // custom modifiers
            Self::HAT,
        ]
    }
}
