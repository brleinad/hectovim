#![warn(clippy::all, clippy::pedantic)]
extern crate core;

mod editor;
use editor::Editor;
pub use editor::Position;

mod terminal;
use terminal::Terminal;

fn main() {
    Editor::default().run();
}
