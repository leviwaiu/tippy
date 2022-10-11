pub mod main_list;
pub mod anime_search;


use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    Frame
};

pub(crate) enum Scenes {
    MainList,
    AnimeSearch,
    AnimeList,
}

pub trait Displayable {
    fn widget<B: Backend>(&mut self, f: &mut Frame<B>);

    fn process_key(&mut self, key:KeyCode);
}

pub trait Connection {

    fn connect_interface();
}
