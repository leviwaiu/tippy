use crate::anilist::interface::AniListInterface;
use crate::scene::anime_search::AnimeSearch;
use crate::scene::mainlist::MainList;
use crate::scene::settings::{Settings, SettingsScene};
use crate::terminal::Terminal;
use termion::event::Key;

pub mod anime_info;
pub mod anime_search;
pub mod mainlist;
pub mod settings;
mod scrollable;

pub(crate) enum Scene {
    MainList(MainList),
    Settings(SettingsScene),
    AnimeSearch(AnimeSearch),
}

pub trait SceneTrait {
    fn show_view(&self, terminal: &Terminal);

    fn format_status_row(&self) -> String;

    fn process_key(&mut self, key: Key, terminal: &Terminal, settings: Settings);

    fn connect_interface(&mut self, interface: &mut AniListInterface);
}

impl SceneTrait for Scene {
    fn show_view(&self, terminal: &Terminal) {
        Terminal::clear_screen();
        match self {
            Scene::MainList(main_list) => main_list.show_view(terminal),
            Scene::Settings(settings) => settings.show_view(terminal),
            Scene::AnimeSearch(anime_search) => anime_search.show_view(terminal),
            _ => (),
        }
    }

    fn format_status_row(&self) -> String {
        match self {
            Scene::MainList(main_list) => main_list.format_status_row(),
            _ => "".to_string(),
        }
    }

    fn process_key(&mut self, key: Key, terminal: &Terminal, settings: Settings) {
        match self {
            Scene::MainList(main_list) => main_list.process_key(key, terminal, settings),
            Scene::AnimeSearch(anime_search) => anime_search.process_key(key, terminal, settings),
            _ => (),
        }
    }

    fn connect_interface(&mut self, interface: &mut AniListInterface) {
        match self {
            Scene::MainList(main_list) => main_list.connect_interface(interface),
            Scene::AnimeSearch(anime_search) => anime_search.connect_interface(interface),
            _ => (),
        }
    }
}
