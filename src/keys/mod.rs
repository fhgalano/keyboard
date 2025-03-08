#![allow(non_camel_case_types)]

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
    PAW,
    // normal keys
	Aa,
	Bb,
	Cc,
    Dd,
    Ee,
	Ff,
	Gg,
	Hh,
	Ii,
	Jj,
	Kk,
	Ll,
	Mm,
	Nn,
	Oo,
	Pp,
	Qq,
	Rr,
	Ss,
	Tt,
	Uu,
	Vv,
	Ww,
	Xx,
	Yy,
	Zz,
    // Numbers and Symbols
	ONE_EXCLAMATION,
	TWO_AT,
	THREE_HASH,
	FOUR_DOLLAR,
	FIVE_PERCENT,
	SIX_HAT,
	SEVEN_AMPERSAND,
	EIGHT_STAR,
	NINE_LPARENTHESIS,
	ZERO_RPARENTHESIS,

	DASH_UNDERSCORE,
	EQUALS_PLUS,
	LSQUARE_LCURLY,
	RSQUARE_RCURYLY,
	BSLASH_BAR,
	SEMICOLON_COLON,
	SQUOTE_DQUOTE,
	GRAVE_TILDE,
	COMMA_LANGLE,
	PERIOD_RANGLE,
	FSLASH_QUESTION,

	// Common Special
	ENTER,
	ESCAPE,
	BACKSPACE,
	TAB,
	SPACE,
	CAPS_LOCK,
	DELETE,
    PRINT_SCREEN,

	// Function
	F1,
	F2,
	F3,
	F4,
	F5,
	F6,
	F7,
	F8,
	F9,
	F10,
	F11,
	F12,

    // Arrows
    LEFT,
    RIGHT,
    UP,
    DOWN,

}

impl Key {
    pub const fn scancode(&self) -> u8 {
        match *self {
            Self::NOKEY => 0x00,

            // modifiers
            Self::LCTRL => 0xE0,
            Self::RCTRL => 0xE4,
            Self::LSHIFT => 0xE1,
            Self::RSHIFT => 0xE5,
            Self::LALT => 0xE2,
            Self::RALT => 0xE6,
            Self::LSUPER => 0xE3,
            Self::RSUPER => 0xE7,

            Self::HAT => CUSTOM_KEY,
            Self::PAW => CUSTOM_KEY,

			Self::Aa => 0x04,
			Self::Bb => 0x05,
			Self::Cc => 0x06,
			Self::Dd => 0x07,
			Self::Ee => 0x08,
			Self::Ff => 0x09,
			Self::Gg => 0x0A,
			Self::Hh => 0x0B,
			Self::Ii => 0x0C,
			Self::Jj => 0x0D,
			Self::Kk => 0x0E,
			Self::Ll => 0x0F,
			Self::Mm => 0x10,
			Self::Nn => 0x11,
			Self::Oo => 0x12,
			Self::Pp => 0x13,
			Self::Qq => 0x14,
			Self::Rr => 0x15,
			Self::Ss => 0x16,
			Self::Tt => 0x17,
			Self::Uu => 0x18,
			Self::Vv => 0x19,
			Self::Ww => 0x1A,
			Self::Xx => 0x1B,
			Self::Yy => 0x1C,
			Self::Zz => 0x1D,
            
            // Numbers and Symbols
			Self::ONE_EXCLAMATION => 0x1E,
			Self::TWO_AT => 0x1F,
			Self::THREE_HASH => 0x20,
			Self::FOUR_DOLLAR => 0x21,
			Self::FIVE_PERCENT => 0x22,
			Self::SIX_HAT => 0x23,
			Self::SEVEN_AMPERSAND => 0x24,
			Self::EIGHT_STAR => 0x25,
			Self::NINE_LPARENTHESIS => 0x26,
			Self::ZERO_RPARENTHESIS => 0x27,

			Self::DASH_UNDERSCORE => 0x2D,
			Self::EQUALS_PLUS => 0x2E,
			Self::LSQUARE_LCURLY => 0x2F,
			Self::RSQUARE_RCURYLY => 0x30,
			Self::BSLASH_BAR => 0x31,
			Self::SEMICOLON_COLON => 0x33,
			Self::SQUOTE_DQUOTE => 0x34,
			Self::GRAVE_TILDE => 0x35,
			Self::COMMA_LANGLE => 0x36,
			Self::PERIOD_RANGLE => 0x37,
			Self::FSLASH_QUESTION => 0x38,

            // Common Special Keys
			Self::ENTER => 0x28,
			Self::ESCAPE => 0x29,
			Self::BACKSPACE => 0x2A,
			Self::TAB => 0x2B,
			Self::SPACE => 0x2C,
			Self::CAPS_LOCK => 0x39,
			Self::DELETE => 0x4C,
            Self::PRINT_SCREEN => 0x46,

            // Function Keys
			Self::F1 => 0x3A,
			Self::F2 => 0x3B,
			Self::F3 => 0x3C,
			Self::F4 => 0x3D,
			Self::F5 => 0x3E,
			Self::F6 => 0x3F,
			Self::F7 => 0x40,
			Self::F8 => 0x41,
			Self::F9 => 0x42,
			Self::F10 => 0x43,
			Self::F11 => 0x44,
			Self::F12 => 0x45,

            // Arrows
            Self::RIGHT => 0x4F,
            Self::LEFT => 0x50,
            Self::DOWN => 0x51,
            Self::UP => 0x52,
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

    pub fn modifiers() -> [Self; 10] {
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
            Self::PAW,
        ]
    }
}
