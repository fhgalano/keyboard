mod keycombo;
mod keyreport;
mod keys;
mod matrix;

use std::boxed::Box;
use std::collections::HashMap;
pub use keycombo::KeyCombo;
pub use keyreport::KeyReport;
pub use keys::Key;
pub use matrix::{ Matrix, MatrixLoc };

pub type Layer = HashMap<MatrixLoc, Key>;

pub struct Keyboard {
    matrix: Box<dyn Matrix>,
    key_map: HashMap<KeyCombo, Layer>,
    modifier_map: Layer, // specific layer for holding modifier keys
    report_queue: Vec<KeyReport>,
    state: Vec<MatrixLoc>,
}

impl Keyboard {
    pub fn new(
        matrix: Box<dyn Matrix>, 
        key_map: HashMap<KeyCombo, Layer>,
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

    pub fn poll(&mut self) {
        let new_state = self.matrix.poll();

        if new_state != self.state || !new_state.is_empty() {
            self.state = new_state;
            self.eval_state();
        }
    }

    fn eval_state(&mut self) {
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
    }

    pub fn give_report(&mut self) -> Option<KeyReport> {
        self.report_queue.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    fn test_layermap() -> HashMap<KeyCombo, Layer> {
        let mut base_map = HashMap::new();
        base_map.insert((0, 1), Key::Dd);
        base_map.insert((0, 2), Key::Ee);
        base_map.insert((1, 1), Key::Ee);
        base_map.insert((1, 2), Key::Zz);

        let mut bigmap = HashMap::new();
        bigmap.insert(KeyCombo::default(), base_map);
        
        bigmap
    }

    fn test_modifier_map() -> Layer {
        let mut mod_layer = HashMap::new();
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
}
