use crate::scene::SceneTrait;
use crate::terminal::Terminal;
use termion::event::Key;
use crate::scene::settings::Settings;
use crate::anilist_interface::AniListInterface;

pub struct AnimeSearch {

}

impl SceneTrait for AnimeSearch {
    fn show_view(&self, terminal: &Terminal) {
        todo!()
    }

    fn format_status_row(&self) -> String {
        todo!()
    }

    fn process_key(&mut self, key: Key, terminal: &Terminal, settings: Settings) {
        todo!()
    }

    fn connect_interface(&mut self, interface: &AniListInterface) {
        todo!()
    }
}