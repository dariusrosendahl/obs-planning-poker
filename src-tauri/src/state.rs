use std::sync::Mutex;

use crate::types::{CardState, CARD_VALUES};

pub struct AppState {
    inner: Mutex<CardState>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(CardState {
                value: CARD_VALUES[0].to_string(),
                revealed: false,
                index: 0,
            }),
        }
    }

    pub fn get(&self) -> CardState {
        self.inner.lock().unwrap().clone()
    }

    pub fn set_card(&self, value: &str) -> CardState {
        let mut state = self.inner.lock().unwrap();
        if let Some(index) = CARD_VALUES.iter().position(|&v| v == value) {
            state.value = value.to_string();
            state.index = index;
        }
        state.clone()
    }

    pub fn next_card(&self) -> CardState {
        let mut state = self.inner.lock().unwrap();
        state.index = (state.index + 1) % CARD_VALUES.len();
        state.value = CARD_VALUES[state.index].to_string();
        state.clone()
    }

    pub fn prev_card(&self) -> CardState {
        let mut state = self.inner.lock().unwrap();
        state.index = (state.index + CARD_VALUES.len() - 1) % CARD_VALUES.len();
        state.value = CARD_VALUES[state.index].to_string();
        state.clone()
    }

    pub fn toggle_reveal(&self) -> CardState {
        let mut state = self.inner.lock().unwrap();
        state.revealed = !state.revealed;
        state.clone()
    }

    pub fn hide_card(&self) -> CardState {
        let mut state = self.inner.lock().unwrap();
        state.revealed = false;
        state.clone()
    }
}
