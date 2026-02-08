use std::sync::Arc; // this is Rust "import"

use winit::{
    // multiple multi-level imports, same as `import winit.application.ApplicationHandler`
    // and so on
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

// Conditional compilation. Line below only runs when
// a compilation flag `target_arch="wasm32"` is set
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// In Rust you define a structure "contents" and
// its methods separately. This thing below only
// defines internal fields, in this case there is only one - `window`
// pub = importable
pub struct State {
    window: Arc<Window>, // `Arc<Window>` implies that `window` is a `Window` that can have
                         // multiple owners
}

// In this block we add methods for the already defined `State` struct.
// You CAN have multiple impl blocks for the same struct, even in the same file,
// but they must be in the same crate (your project). You cannot add methods
// to structs defined in external crates.
impl State {
    // We don't need this to be async right now,
    // but we will in the next tutorial
    //
    // pub = public, can be called from outside this module
    // fn = function
    // async = returns a Future, must be .await'ed
    // new = function name (convention for constructors, not special)
    // anyhow::Result<Self> expands to Result<Self, anyhow::Error>
    //   - Result = enum with Ok(T) or Err(E) variants
    //   - Ok holds Self (the State struct)
    //   - Err holds anyhow::Error (pre-filled by anyhow crate)
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        // In Rust you can have an implicit return by just leaving an expression
        // on last executed line without `;`
        // In this case, we return Ok (which is a possible variant of Result) carrying
        // `Self` (so an instance of struct State) that has its field `window` set to variable
        // `window` that pas passed as input to `new()`
        Ok(Self { window })
    }

    pub fn resize(&mut self, _width: u32, _height: u32) {
        // We'll do stuff here in the next tutorial
    }

    pub fn render(&mut self) {
        self.window.request_redraw();

        // We'll do more stuff here in the next tutorial
    }
}
