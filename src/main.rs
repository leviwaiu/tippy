use tippy::Tippy;
use crate::new_terminal::TerminalInterface;
use crate::new_scene::test_struct::TestStruct;

use std::thread;
use std::time::Duration;
use tui::backend::{Backend, CrosstermBackend};
use tui::Frame;
use tui::widgets::{Block, Borders};
use crate::new_scene::{NewSceneTrait, test_struct};

mod anilist;
mod anime_entry;
mod list_entry;
mod scene;
mod secrets;
mod terminal;
mod tippy;
mod new_terminal;
mod new_scene;


fn main() {
    Tippy::default().run()

    //
    // let mut term = TerminalInterface::default().unwrap();
    //
    // let test_struct = TestStruct::default();
    // let mut widget_function = |f:&mut Frame<CrosstermBackend<std::io::Stdout>>| test_struct.widget(f);
    //
    // term.render_widget(widget_function).expect("TODO: panic message");
    //
    // thread::sleep(Duration::from_millis(5000));
    //
    // term.restore_terminal().expect("TODO: panic message");

}
