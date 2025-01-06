use super::keys::Key;
use std::vec::Vec;

#[derive(Debug, Default, Clone, Copy, Eq, Hash, PartialEq)]
pub struct KeyCombo {
    key_0: Option<Key>,
    key_1: Option<Key>,
    key_2: Option<Key>
}

impl KeyCombo {
    pub fn new(keys: &[Key]) -> Self {
        Self {
            key_0: keys.get(0).copied(),
            key_1: keys.get(1).copied(),
            key_2: keys.get(2).copied(),
        }
    }

    pub fn detect(&self, keys: &[Key]) -> bool {
        self.keys().iter().all(|k| keys.contains(k))
    }

    fn keys(&self) -> Vec<Key> {
        let real_keys: Vec<_> = [self.key_0, self.key_1, self.key_2].iter().filter_map(|x| *x).collect();

        real_keys
    }

    pub fn size(&self) -> usize {
        self.keys().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_combo() -> KeyCombo {
        KeyCombo::new(&[Key::LSHIFT, Key::Dd])
    }
    
    #[test]
    fn detect_default_in_empty_array() {
        assert!(KeyCombo::default().detect(&[]));
    }

    #[test]
    fn detect_default_in_array() {
        assert!(KeyCombo::default().detect(&[Key::Dd, Key::Ee]))
    }

    #[test]
    fn detect_test_combo() {
        assert!(test_combo().detect(&[Key::Dd, Key::Ee, Key::LSHIFT]))
    }

    #[test]
    fn fail_combo_detection() {
        assert!(!test_combo().detect(&[Key::LSHIFT]))
    }
}

