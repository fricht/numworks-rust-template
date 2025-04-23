#![no_std]

extern crate alloc;

use alloc::{
    boxed::Box,
    format,
    string::{String, ToString},
};
use graphmgr::{StackAction, State, StateManager};
use libnw::{
    display::{self, CHAR_HEIGHT, Color, LARGE_CHAR_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH},
    keyboard::{KeyboardTimedState, RawKey},
};

/// How to handle the end of the main app.
pub enum ExitBehaviour {
    Exit,
    Hang,
    Restart,
}

struct MainState {
    pub msg: String,
}

impl State<()> for MainState {
    fn update(&mut self, keyboard_state: &KeyboardTimedState) -> StackAction<()> {
        if keyboard_state.is_key_just_pressed(RawKey::Ok) {
            StackAction::Push(Box::new(MainState {
                msg: format!("+{}", self.msg),
            }))
        } else if keyboard_state.is_key_just_pressed(RawKey::Back) {
            StackAction::Pop(())
        } else {
            StackAction::Nop
        }
    }

    fn render(&mut self) {
        display::eadk::wait_for_vblank();
        display::clear_screen(Color::GREEN);
        display::draw_string(
            &self.msg,
            (SCREEN_WIDTH - (self.msg.len() as u16 * LARGE_CHAR_WIDTH)) / 2,
            (SCREEN_HEIGHT - CHAR_HEIGHT) / 2,
            true,
            Color::BLACK,
            Color::WHITE,
        );
    }
}

/// The core of the application logic
pub fn main() -> ExitBehaviour {
    let mut state_mgr = StateManager::<()>::new();
    state_mgr.run(
        Box::new(MainState {
            msg: "Hey !!".to_string(),
        }),
        60,
    );
    ExitBehaviour::Exit
}
