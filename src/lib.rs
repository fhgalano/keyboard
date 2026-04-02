#![no_std]

mod keycombo;
mod keyreport;
mod keys;
mod matrix;

extern crate alloc;

use alloc::{
    boxed::Box,
    vec::Vec,
};
use heapless::index_map::FnvIndexMap;

pub use keycombo::KeyCombo;
pub use keyreport::KeyReport;
pub use keys::Key;
pub use matrix::{ Matrix, MatrixLoc, MatrixLoc2 };

pub type Layer = FnvIndexMap<MatrixLoc, Key, 128>;

pub struct Keyboard {
    matrix: Box<dyn Matrix>,
    key_map: FnvIndexMap<KeyCombo, Layer, 4>,
    modifier_map: Layer, // specific layer for holding modifier keys
    report_queue: Vec<KeyReport>,
    state: Vec<MatrixLoc>,
}

impl Keyboard {
    pub fn new(
        matrix: Box<dyn Matrix>, 
        key_map: FnvIndexMap<KeyCombo, Layer, 4>,
        modifier_map: Layer, // specific layer for holding modifier keys
    ) -> Self {
        Self {
            matrix,
            key_map,
            modifier_map,
            report_queue: Vec::new(),
            state: Vec::new(),
        }
    }

    pub fn poll(&mut self) -> Option<State> {
        let new_state = self.matrix.poll();

        if new_state != self.state || !new_state.is_empty() {
            self.state = new_state;
            return Some(self.eval_state())
        }
        None
    }

    fn eval_state(&mut self) -> State {
        // check for modifier keys
        let pressed_modifiers: Vec<_> = self.state
            .iter()
            .filter_map(|key_loc| self.modifier_map.get(key_loc))
            .copied()
            .collect();

        // determine which key layer should be referenced
        let active_combo = self.key_map
            .keys()
            .filter(|kc| kc.detect(&pressed_modifiers))
            .max_by(|kc1, kc2| kc1.size().cmp(&kc2.size()))
            .copied()
            .unwrap_or_default();
        let active_layer = self.key_map.get(&active_combo).unwrap();

        // get keys from the correct map
        let keys: Vec<Key> = self.state
            .iter()
            .filter_map(|key_loc| active_layer.get(key_loc).copied())
            .collect();

        // add report to the queue if valid
        if let Ok(kr) = KeyReport::new_from_keys(&keys, &pressed_modifiers) {
            self.report_queue.push(kr);
        };

        State::new(keys, pressed_modifiers)
    }

    pub fn give_report(&mut self) -> Option<KeyReport> {
        self.report_queue.pop()
    }
}

// note, MatrixLoc are row,col format (i think)
#[allow(unused)]
#[macro_export]
macro_rules! layer {
    (
        $([$($key:expr),+]),+ $(,)?
    ) =>{
        {
            let mut layer: Layer = heapless::index_map::FnvIndexMap::new();
            let mut row: u8 = 0;
            $(
                let mut col: u8 = 0;
                $(
                    if $key != Key::NOKEY {
                        layer.insert((row, col), $key);
                    }
                    col += 1;
                )+
                row += 1;
            )+

            layer
        }
    };
}

#[derive(Debug)]
pub struct State {
    pub modifiers: Vec<Key>,
    pub keys: Vec<Key>,
}

impl State {
    pub fn new(keys: Vec<Key>, modifiers: Vec<Key>) -> Self {
        Self {
            keys,
            modifiers,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::vec;

    struct TestMatrix {
        state: Vec<MatrixLoc>,
        pos: usize,
        states: Vec<Vec<MatrixLoc>>,
    }

    impl TestMatrix {
        pub fn new(states: Vec<Vec<MatrixLoc>>) -> Self {
            Self {
                state: Vec::new(),
                pos: 0,
                states,
            }
        }

        fn set_state(&mut self, state: Vec<MatrixLoc>) {
            self.state = state;
        }
    }

    impl Matrix for TestMatrix {
        fn poll(&mut self) -> Vec<MatrixLoc> {
            self.set_state(self.states[self.pos].clone());
            match self.pos == self.states.len() - 1 {
                true => self.pos = 0,
                false => self.pos += 1,
            };
            self.state.clone()
        }
    }

    fn test_layermap() -> FnvIndexMap<KeyCombo, Layer, 4> {
        let mut base_map = FnvIndexMap::new();
        base_map.insert((0, 1), Key::Dd);
        base_map.insert((0, 2), Key::Ee);
        base_map.insert((1, 1), Key::Ee);
        base_map.insert((1, 2), Key::Zz);

        let mut bigmap = FnvIndexMap::new();
        bigmap.insert(KeyCombo::default(), base_map);
        
        bigmap
    }

    fn test_modifier_map() -> Layer {
        let mut mod_layer = FnvIndexMap::new();
        mod_layer.insert((0, 0), Key::LSHIFT);
        mod_layer.insert((1, 0), Key::LCTRL);

        mod_layer
    }

    // map layout:
    // SHIFT    D   E
    // CTRL     E   Z
    fn test_keyboard(matrix: TestMatrix) -> Keyboard {
        Keyboard::new(
            Box::new(matrix),
            test_layermap(),
            test_modifier_map(),
        )
    }

    #[test]
    fn poll_different_states() {
        let test_states = vec![
            vec![(0, 0), (0, 1)],
            vec![(0, 2)],
            vec![(0, 2)],
            vec![(1, 2)],
        ];

        let mut kbd = test_keyboard(TestMatrix::new(test_states.clone()));

        for _ in test_states {
            kbd.poll();
        }

        assert_eq!(
            kbd.report_queue,
            vec![
                KeyReport::new(0b0000_0010, [0x07, 0x00, 0x00, 0x00, 0x00, 0x00]),
                KeyReport::new(0b0000_0000, [0x08, 0x00, 0x00, 0x00, 0x00, 0x00]),
                KeyReport::new(0b0000_0000, [0x08, 0x00, 0x00, 0x00, 0x00, 0x00]),
                KeyReport::new(0b0000_0000, [0x1D, 0x00, 0x00, 0x00, 0x00, 0x00]),
            ]
        )
    }

    #[test]
    fn poll_same_active_states() {
        let test_states = vec![
            vec![(0, 0), (0, 1)],
            vec![(0, 0), (0, 1)],
            vec![(0, 0), (0, 1)],
            vec![(0, 0), (0, 1)],
        ];

        let mut kbd = test_keyboard(TestMatrix::new(test_states.clone()));

        for _ in test_states {
            kbd.poll();
        }

        assert_eq!(
            kbd.report_queue,
            vec![
                KeyReport::new(0b0000_0010, [0x07, 0x00, 0x00, 0x00, 0x00, 0x00]),
                KeyReport::new(0b0000_0010, [0x07, 0x00, 0x00, 0x00, 0x00, 0x00]),
                KeyReport::new(0b0000_0010, [0x07, 0x00, 0x00, 0x00, 0x00, 0x00]),
                KeyReport::new(0b0000_0010, [0x07, 0x00, 0x00, 0x00, 0x00, 0x00]),
            ]
        )
    }

    #[test]
    fn poll_empty_states() {
        let test_states = vec![
            vec![(0, 0), (0, 1)],
            vec![],
            vec![],
            vec![],
        ];

        let mut kbd = test_keyboard(TestMatrix::new(test_states.clone()));

        for _ in test_states {
            kbd.poll();
        }

        assert_eq!(
            kbd.report_queue,
            vec![
                KeyReport::new(0b0000_0010, [0x07, 0x00, 0x00, 0x00, 0x00, 0x00]),
                KeyReport::new(0b0000_0000, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
            ]
        )
    }

    #[test]
    fn test_simple_layer() {
        let test_layer = layer!(
            [Key::Dd, Key::Ee, Key::Ee, Key::Zz],
            [Key::Nn, Key::Uu, Key::Tt, Key::Ss],
            [Key::Hh, Key::NOKEY, Key::NOKEY, Key::Ii],
        );

        let mut expected_layer: Layer = FnvIndexMap::new();
        // row 1
        expected_layer.insert((0,0), Key::Dd);
        expected_layer.insert((0,1), Key::Ee);
        expected_layer.insert((0,2), Key::Ee);
        expected_layer.insert((0,3), Key::Zz);
        
        // row 2
        expected_layer.insert((1,0), Key::Nn);
        expected_layer.insert((1,1), Key::Uu);
        expected_layer.insert((1,2), Key::Tt);
        expected_layer.insert((1,3), Key::Ss);

        // row 3
        expected_layer.insert((2,0), Key::Hh);
        expected_layer.insert((2,3), Key::Ii);

        assert_eq!(test_layer, expected_layer);
    }
}
