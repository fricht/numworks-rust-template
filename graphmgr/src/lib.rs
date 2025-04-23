#![no_std]

extern crate alloc;

use alloc::{boxed::Box, vec::Vec};
use libnw::{keyboard::KeyboardTimedState, time};

/// Represents an action to apply on the states stack.
///
/// `M` is a pop message (transferred to the state below).
///
/// Should only be used with `StateManager`.
pub enum StackAction<M = ()> {
    Pop(M),
    Push(Box<dyn State<M>>),
    Replace(Box<dyn State<M>>),
    Nop,
}

/// The state manager.
///
/// # Example :
/// (too basic, todo : make it better)
/// ```
/// let mut state_mgr = StateManager::<()>::new();
/// state_mgr.run(Box::new(MyState::new()));
/// ```
pub struct StateManager<M = ()> {
    stack: Vec<Box<dyn State<M>>>,
}

impl<M> StateManager<M> {
    /// Creates a new instance.
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    // should i make these methods public ???

    fn push_to_stack(&mut self, mut state: Box<dyn State<M>>) {
        if let Some(f) = self.stack.last_mut() {
            match f.pause() {
                StackAction::Pop(msg) => {
                    self.pop_from_stack(msg);
                }
                StackAction::Push(state) => {
                    self.push_to_stack(state);
                }
                StackAction::Replace(state) => {
                    self.replace_top_stack(state);
                }
                StackAction::Nop => (),
            }
        }
        match state.create() {
            StackAction::Pop(msg) => {
                self.pop_from_stack(msg);
            }
            StackAction::Push(state) => {
                self.push_to_stack(state);
            }
            StackAction::Replace(state) => {
                self.replace_top_stack(state);
            }
            StackAction::Nop => (),
        }
        self.stack.push(state);
    }

    fn pop_from_stack(&mut self, msg: M) -> Option<Box<dyn State<M>>> {
        let mut poped_frame = self.stack.pop();
        match &mut poped_frame {
            Some(f) => match f.quit() {
                StackAction::Pop(msg) => {
                    self.pop_from_stack(msg);
                }
                StackAction::Push(state) => {
                    self.push_to_stack(state);
                }
                StackAction::Replace(state) => {
                    self.replace_top_stack(state);
                }
                StackAction::Nop => (),
            },
            None => return None,
        }
        if let Some(f) = self.stack.last_mut() {
            match f.resume(msg) {
                StackAction::Pop(msg) => {
                    self.pop_from_stack(msg);
                }
                StackAction::Push(state) => {
                    self.push_to_stack(state);
                }
                StackAction::Replace(state) => {
                    self.replace_top_stack(state);
                }
                StackAction::Nop => (),
            }
        }
        poped_frame
    }

    fn replace_top_stack(&mut self, mut state: Box<dyn State<M>>) -> Option<Box<dyn State<M>>> {
        let mut poped_frame = self.stack.pop();
        if let Some(f) = &mut poped_frame {
            match f.quit() {
                StackAction::Pop(msg) => {
                    self.pop_from_stack(msg);
                }
                StackAction::Push(state) => {
                    self.push_to_stack(state);
                }
                StackAction::Replace(state) => {
                    self.replace_top_stack(state);
                }
                StackAction::Nop => (),
            }
        }

        match state.create() {
            StackAction::Pop(msg) => {
                self.pop_from_stack(msg);
            }
            StackAction::Push(state) => {
                self.push_to_stack(state);
            }
            StackAction::Replace(state) => {
                self.replace_top_stack(state);
            }
            StackAction::Nop => (),
        }
        self.stack.push(state);
        poped_frame
    }

    // --------

    /// Here we go !!!\
    /// (with initial state)
    pub fn run(&mut self, initial_state: Box<dyn State<M>>, fps: u64) {
        let time_interval = 1000 / fps;
        let mut kb_handler = KeyboardTimedState::new();
        self.push_to_stack(initial_state);
        let mut now = time::monotonic();
        while let Some(frame) = self.stack.last_mut() {
            kb_handler.fetch();
            match frame.update(&kb_handler) {
                StackAction::Pop(msg) => {
                    self.pop_from_stack(msg);
                    continue;
                }
                StackAction::Push(state) => {
                    self.push_to_stack(state);
                    continue;
                }
                StackAction::Replace(state) => {
                    self.replace_top_stack(state);
                    continue;
                }
                StackAction::Nop => (),
            }
            frame.render();
            let new_now = time::monotonic();
            let elapsed = new_now - now;
            if elapsed < time_interval {
                time::msleep((time_interval - elapsed) as u32);
            }
            now = time::monotonic();
        }
    }
}

pub trait State<M = ()> {
    /// called when adding state to stack
    fn create(&mut self) -> StackAction<M> {
        StackAction::Nop
    }
    /// called when another state is pushed on top
    fn pause(&mut self) -> StackAction<M> {
        StackAction::Nop
    }
    /// called when state on top is poped
    fn resume(&mut self, _pop_message: M) -> StackAction<M> {
        StackAction::Nop
    }
    /// called when state is poped
    fn quit(&mut self) -> StackAction<M> {
        StackAction::Nop
    }
    /// when frame is active, called frequently\
    /// intended for logic update
    fn update(&mut self, keyboard_state: &KeyboardTimedState) -> StackAction<M>;
    /// when frame is active called frequently\
    /// intended for render logic
    fn render(&mut self);
}
