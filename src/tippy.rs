use crate::terminal::{Terminal, Position};
use termion::event::Key;

use crate::entry::{Entry, EntryStatus};
use crate::anilist_interface::AniListInterface;
use unicode_width::UnicodeWidthStr;
use crate::scene::settings::Settings;
use crate::scene::{SceneTrait, Scene};
use crate::scene::mainlist::MainList;

pub struct Tippy {
    terminal: Terminal,
    anime_list: Vec<Entry>,
    quit: bool,
    interface: AniListInterface,
    selected: Position,
    offset: Position,

    scene: Scene,

    settings: Settings,
}

impl Tippy{
    pub fn default() -> Self {
        let out = Self {
            terminal: Terminal::default().expect("Terminal Initialisation Failed"),
            anime_list: Vec::new(),
            quit: false,
            interface: AniListInterface::default(),
            selected: Position{ x: 0, y: 0 },
            offset: Position::default(),

            scene: Scene::MainList(MainList::default()),
            settings: Settings::default(),

        };
        out
    }
    pub fn run(&mut self) {

        self.setup();

        loop {
            if let Err(error) = self.process_screen_tick() {
                die(&error);
            }
            if self.quit{
                break;
            }
            if let Err(error) = self.process_keypresses() {
                die(&error);
            }
        }
    }
    fn process_screen_tick(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position::default());
        if self.quit {
            Terminal::clear_screen();
            Terminal::cursor_show();
            println!("Exiting...\r");
        }
        else {
            self.scene.show_view(&self.terminal);
            Terminal::cursor_position(&Position {
                x: self.selected.x.saturating_sub(self.offset.x),
                y: self.selected.y.saturating_sub(self.offset.y) + 1,
            })
        }
        Terminal::flush()
    }
    fn process_keypresses(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Char('q') => self.quit = true,
            // Key::Up
            // | Key::Down
            // | Key::PageUp
            // | Key::PageDown => self.move_cursor(pressed_key),
            // Key::Char('+')
            // | Key::Char('-') => self.edit_entry(pressed_key),
            _ => self.scene.process_key(pressed_key, &self.terminal, &self.settings),
        }

        Ok(())
    }
    fn scroll(&mut self){
        let Position {x:_, y} = self.selected;
        let _width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;
        let mut offset = &mut self.offset;

        if y <= offset.y {
            offset.y = y;
        }
        else if y >= offset.y.saturating_add(height - 2) {
            offset.y = y.saturating_sub(height - 2).saturating_add(1);
        }
    }
    fn move_cursor(&mut self, key:Key){
        let terminal_height = self.terminal.size().height as usize;
        let Position {x, mut y} = self.selected;
        let list_length = self.anime_list.len();

        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down =>
                if y < list_length.saturating_sub(1) {
                    y = y.saturating_add(1);
                },
            Key::PageUp => {
                y = if y > terminal_height {
                    y.saturating_sub(terminal_height)
                } else {
                    0
                }
            }
            Key::PageDown => {
                y = if y.saturating_add(terminal_height) < list_length {
                    y.saturating_add(terminal_height)
                } else {
                    list_length
                }
            }
            _ => ()
        }

        self.selected = Position {x, y}
    }
    fn edit_entry(&mut self, key:Key){
        let selected_no = self.selected.y;
        match key {
            Key::Char('+') => {
                if self.anime_list[selected_no].watched_count() == 0
                    && self.anime_list[selected_no].status() == EntryStatus::PLANNING
                    && self.settings.auto_change_status()
                {
                    self.anime_list[selected_no].set_status(EntryStatus::CURRENT);
                }
                self.anime_list[selected_no].add_watched()
            },
            Key::Char('-') => self.anime_list[selected_no].remove_watched(),
            _ => (),
        }
        self.interface.edit_anime_watchcount(self.anime_list[selected_no].clone());

    }
    fn setup(&mut self){

        self.terminal.put_into_raw().expect("Terminal Initialisation Failed");

        self.interface.authentication();
        self.interface.fetch_viewer();
        if let Scene::MainList(main_list) = &mut self.scene {
            main_list.set_anime_list(self.interface.fetch_anime_list());
        }
        //self.main_list.set_anime_list(&mut self.interface);
        //REMOVE WHEN NEEDED
        //self.terminal.debug_size_override();
    }

}

fn die(e: &std::io::Error){
    Terminal::clear_screen();
    panic!("{}",e);
}