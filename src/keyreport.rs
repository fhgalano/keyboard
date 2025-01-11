use super::keys::Key;
use anyhow::{ anyhow, Result };
use zerocopy_derive::{ IntoBytes, Immutable };

// structure for keyboard reports as defined by HID
#[derive(Debug, Default, PartialEq, IntoBytes, Immutable)]
#[repr(packed)]
pub struct KeyReport {
    modifiers: u8,
    reserved: u8,
    keys: [u8; 6],
}

impl KeyReport {
    pub fn new(modifiers: u8, keys: [u8; 6]) -> Self {
        Self {
            modifiers,
            reserved: 0x00,
            keys,
        }
    }

    pub fn new_from_key(key: Key, modifiers: &[Key]) -> Result<Self> {
        Ok(Self {
            modifiers: Self::modifier_byte_from_keys(modifiers)?,
            reserved: 0x00,
            keys: [key.scancode(), 0x00, 0x00, 0x00, 0x00, 0x00],
        })
    }

    pub fn new_from_keys(keys: &[Key], modifiers: &[Key]) -> Result<Self> {
        match keys.len() > 6 {
            true => Err(anyhow!("this dude be mashing (pressing more than 6 keys at once)")),
            false => Ok(Self {
                modifiers: Self::modifier_byte_from_keys(modifiers)?,
                reserved: 0x00, 
                keys: [0, 1, 2, 3, 4, 5]
                    .map(|idx| keys.get(idx).unwrap_or(&Key::NOKEY).scancode())
            })
        }
    }

    fn modifier_byte_from_keys(modifiers: &[Key]) -> Result<u8> {
        if modifiers.iter().any(|k| !Key::modifier_scancodes().contains(&k.scancode())) {
            return Err(anyhow!("cannot use non-modifier keys"))
        }

        let mut modifier_byte: u8 = 0;
        for key in modifiers {
            match key {
                Key::LCTRL => modifier_byte |= 0b1000_0000,
                Key::LSHIFT => modifier_byte |= 0b0100_0000,
                Key::LALT => modifier_byte |= 0b0010_0000,
                Key::LSUPER => modifier_byte |= 0b0001_0000,
                Key::RCTRL => modifier_byte |= 0b0000_1000,
                Key::RSHIFT => modifier_byte |= 0b0000_0100,
                Key::RALT => modifier_byte |= 0b0000_0010,
                Key::RSUPER => modifier_byte |= 0b0000_0001,
                _ => (),
            }
        }
        
        Ok(modifier_byte)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_key_no_modifiers() {
        let kr = KeyReport::new_from_key(Key::Dd, &[]).unwrap();
        assert_eq!(
            kr, 
            KeyReport {
                modifiers: 0x00,
                reserved: 0x00,
                keys: [0x07, 0x00, 0x00, 0x00, 0x00, 0x00],
            }
        )
    }

    #[test]
    fn from_key_with_modifiers() {
        let kr = KeyReport::new_from_key(Key::Dd, &[Key::LCTRL]).unwrap();
        assert_eq!(
            kr, 
            KeyReport {
                modifiers: 0b1000_0000,
                reserved: 0x00,
                keys: [0x07, 0x00, 0x00, 0x00, 0x00, 0x00],
            }
        )
    }

    #[test]
    // TODO: eventually implement ignoring custom key code
    fn from_custom_key() {
        let kr = KeyReport::new_from_key(Key::HAT, &[Key::LCTRL]).unwrap();
        assert_eq!(
            kr, 
            KeyReport {
                modifiers: 0b1000_0000,
                reserved: 0x00,
                keys: [0xFF, 0x00, 0x00, 0x00, 0x00, 0x00],
            }
        )
    }
    
    #[test]
    fn from_empty_keys_no_modifiers() {
        let kr = KeyReport::new_from_keys(&[], &[]).unwrap();
        assert_eq!(
            kr, 
            KeyReport::default()
        )
    }

    #[test]
    fn from_keys_with_modifiers() {
        let kr = KeyReport::new_from_keys(&[Key::Dd, Key::Ee], &[Key::LSHIFT]).unwrap();
        assert_eq!(
            kr, 
            KeyReport {
                modifiers: 0b0100_0000,
                reserved: 0x00,
                keys: [0x07, 0x08, 0x00, 0x00, 0x00, 0x00],
            }
        )
    }

    #[test]
    #[should_panic]
    // TODO: Duplicate keys should be filtered eventually
    fn from_too_many_keys() {
        KeyReport::new_from_keys(
            &[Key::Dd, Key::Dd, Key::Dd, Key::Dd, Key::Dd, Key::Dd, Key::Dd], &[]
        ).unwrap();
    }

    #[test]
    #[should_panic]
    fn failed_modifier() {
        KeyReport::modifier_byte_from_keys(&[Key::LALT, Key::Dd]).unwrap();
    }

    #[test]
    fn no_modifiers() {
        let modifier_byte = KeyReport::modifier_byte_from_keys(&[]).unwrap();
        assert_eq!(modifier_byte, 0b0000_0000);
    }

    #[test]
    fn double_modifiers() {
        let modifier_byte = KeyReport::modifier_byte_from_keys(&[Key::LALT, Key::LALT, Key::RALT]).unwrap();
        assert_eq!(modifier_byte, 0b0010_0010);
    }

    #[test]
    fn all_modifiers() {
        let modifier_byte = KeyReport::modifier_byte_from_keys(
            &Key::modifiers()
        ).unwrap();
        assert_eq!(modifier_byte, 0b1111_1111);
    }

    #[test]
    fn ctrl_alt_active() {
        let modifier_byte = KeyReport::modifier_byte_from_keys(&[Key::LCTRL, Key::LALT]).unwrap();
        assert_eq!(modifier_byte, 0b1010_0000);
    }
}
