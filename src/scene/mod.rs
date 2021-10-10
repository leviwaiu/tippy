use crate::scene::mainlist::MainList;
use crate::terminal::Terminal;
use termion::event::Key;
use crate::scene::settings::{SettingsScene, Settings};
use crate::anilist_interface::AniListInterface;

pub mod mainlist;
pub mod settings;
mod anime_info;

pub(crate) enum Scene {
    MainList(MainList),
    Settings(SettingsScene),
}

pub trait SceneTrait{
    fn show_view(&self, terminal:&Terminal);

    fn format_status_row(&self) -> String;

    fn process_key(&mut self, key:Key, terminal: &Terminal, settings:Settings);

    fn connect_interface(&mut self, interface: &AniListInterface);

}

impl SceneTrait for Scene {
    fn show_view(&self, terminal: &Terminal) {
        Terminal::clear_screen();
        match self{
            Scene::MainList(main_list) => main_list.show_view(terminal),
            Scene::Settings(settings) => settings.show_view(terminal),
        }
    }

    fn format_status_row(&self) -> String {
        match self {
            Scene::MainList(main_list) => main_list.format_status_row(),
            _ => "".to_string(),
        }
    }

    fn process_key(&mut self, key:Key, terminal: &Terminal, settings:Settings) {
        match self {
            Scene::MainList(main_list) => main_list.process_key(key, terminal, settings),
            _ => (),
        }
    }

    fn connect_interface(&mut self, interface: &AniListInterface) {
        match self {
            Scene::MainList(main_list) => main_list.connect_interface(interface),
            _ => (),
        }
    }
}