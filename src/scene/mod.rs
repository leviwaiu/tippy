pub mod main_list;
pub mod anime_search;
mod anime_details;
mod settings;


use std::io::Stdout;
use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    Frame
};
use tui::backend::CrosstermBackend;
use crate::anilist::interface::AniListInterface;

pub(crate) enum Scenes {
    MainList,
    AnimeSearch,
}

pub trait Displayable {
    fn widget(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>);

    fn process_key(&mut self, key:KeyCode);

    fn connect_interface(&mut self, interface:&AniListInterface);
}

pub trait Scrollable{
    fn move_next(&mut self);

    fn move_prev(&mut self);
}

