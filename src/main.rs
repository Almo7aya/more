#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;
mod document;
mod row;

use editor::Editor;
pub use terminal::Terminal;
pub use editor::Position;
pub use row::Row;
pub use document::Document;

fn main() {
    Editor::default().run();
}
