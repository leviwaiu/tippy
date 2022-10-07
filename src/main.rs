use tippy::Tippy;
use crate::new_terminal::TerminalInterface;
use crate::new_scene::anime_search::AnimeSearch;

use std::thread;
use std::time::Duration;
use tui::backend::{Backend, CrosstermBackend};
use tui::Frame;
use tui::widgets::{Block, Borders};
use crate::anilist::interface::AniListInterface;
use crate::new_scene::{NewSceneTrait, anime_search};
use crate::new_scene::main_list::MainList;

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
    // Tippy::default().run()

    let mut interface = AniListInterface::default();
    interface.authentication();
    interface.fetch_viewer().expect("Error: Failed Fetch_Viewer");

    let mut term = TerminalInterface::default().unwrap();

    let mut test_struct = MainList::default();

    test_struct.set_anime_list(interface.fetch_anime_list(test_struct.get_current_sort()));
    test_struct.set_widget_strings();

    let mut widget_function = |f:&mut Frame<CrosstermBackend<std::io::Stdout>>| test_struct.widget(f);
    term.render_widget(widget_function).expect("TODO: panic message");

    thread::sleep(Duration::from_millis(5000));

    term.restore_terminal().expect("TODO: panic message");

}
