use crate::scene::mainlist::MainList;
use crate::terminal::Terminal;
use termion::event::Key;
use crate::scene::settings::Settings;
use crate::anilist_interface::AniListInterface;

pub mod mainlist;
pub mod settings;
mod anime_info;

pub(crate) enum Scene {
    MainList(MainList),
}

pub trait SceneTrait{
    fn show_view(&self, terminal:&Terminal);

    fn format_status_row(&self) -> String;

    fn process_key(&mut self, key:Key, terminal: &Terminal, settings:&Settings);

    fn connect_interface(&mut self, interface: &AniListInterface);

}

impl SceneTrait for Scene {
    fn show_view(&self, terminal: &Terminal) {
        match self{
            Scene::MainList(main_list) => main_list.show_view(terminal),
        }
    }

    fn format_status_row(&self) -> String {
        match self {
            Scene::MainList(main_list) => main_list.format_status_row(),
        }
    }

    fn process_key(&mut self, key:Key, terminal: &Terminal, settings:&Settings) {
        match self {
            Scene::MainList(main_list) => main_list.process_key(key, terminal, settings),
        }
    }

    fn connect_interface(&mut self, interface: &AniListInterface) {
        match self {
            Scene::MainList(main_list) => main_list.connect_interface(interface),
        }
    }
}