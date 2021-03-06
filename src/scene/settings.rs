use crate::scene::SceneTrait;
use crate::terminal::Terminal;
use termion::event::Key;
use crate::anilist_interface::AniListInterface;
use termion::color;

#[derive(Clone)]
pub struct Settings {
    title_style: String,
    auto_change_status: bool,
}

pub struct SettingsScene {
    settings: Settings,
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

impl SceneTrait for SettingsScene {
    fn show_view(&self, terminal: &Terminal) {
        Terminal::println_bgcolor(&*self.format_title(terminal), Box::new(color::Blue));
        self.print_settingline();
    }

    fn format_status_row(&self) -> String {
        todo!()
    }

    fn process_key(&mut self, _key:Key, _terminal: &Terminal, _settings: Settings) {
        todo!()
    }

    fn connect_interface(&mut self, _interface: &AniListInterface) {
        todo!()
    }
}

impl SettingsScene {

    pub fn default() -> Self {
        Self{
            settings:Settings::default(),
        }
    }

    fn format_title(&self, terminal: &Terminal) -> String{
        let width = terminal.size().width as usize;
        let str = "Settings";

        format!("{}{}", str , " ".repeat(width - str.len()))
    }

    pub fn get_settings(&self) -> Settings {
        self.settings.clone()
    }

    fn print_settingline(&self){
        Terminal::print_fgcolor("Title Style:", Box::new(color::Blue));
    }

}