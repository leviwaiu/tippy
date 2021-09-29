use crate::scene::SceneTrait;
use crate::terminal::Terminal;
use termion::event::Key;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static!{
    static ref SETTINGS: Mutex<Settings> = Mutex::new(
        Settings {
            title_style: String::from("native"),
            auto_change_status:true,
        }
    );
}

pub struct Settings {
    title_style: String,
    auto_change_status: bool,
}

impl SceneTrait for Settings {
    fn show_view(&self, terminal: &Terminal) {

    }

    fn format_status_row(&self) -> String {
        todo!()
    }

    fn process_key(&mut self, key:Key, terminal: &Terminal, settings:&Settings) {
        todo!()
    }
}

impl Settings {
    pub fn default() -> Self {
        Self {
            title_style: String::from("native"),
            auto_change_status: true,
        }
    }


    pub fn auto_change_status(&self) -> bool {
        self.auto_change_status
    }
}