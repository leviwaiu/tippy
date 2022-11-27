use termion::event::Key;

use crate::anilist::interface::AniListInterface;

use std::rc::Rc;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use crate::new_scene::{
    main_list::MainList as NewMainList,
    anime_search::AnimeSearch as NewAnimeSearch,
    Displayable,
    Scenes,
};
use crate::new_terminal::TerminalInterface;

pub struct Tippy {
    new_terminal: TerminalInterface,
    quit: bool,
    interface: AniListInterface,

    curr_scene: Scenes,
    new_main_list: NewMainList,
    new_anime_search: NewAnimeSearch,
}

impl Tippy {
    pub fn default() -> Self {

        let out = Self {

            new_terminal: TerminalInterface::default().expect("New Terminal Interface Initialisation Failed"),

            quit: false,
            interface: AniListInterface::default(),

            curr_scene: Scenes::MainList,
            new_main_list: NewMainList::default(),
            new_anime_search: NewAnimeSearch::default(),

        };
        out
    }

    pub fn run(&mut self) {
        self.setup();

        loop {
            match &self.curr_scene {
                Scenes::MainList => {
                    let x = &mut self.new_main_list;
                    self.new_terminal.render_widget(|f| x.widget(f)).expect("TODO: panic message");
                },
                Scenes::AnimeSearch => {
                    let x = &mut self.new_anime_search;
                    self.new_terminal.render_widget(|f| x.widget(f)).expect("TODO: panic message");
                },
                _ => {}
            };

            if let Err(error) = self.process_keypresses() {
                die(&error);
            }
            if self.quit {
                break;
            }
        }

        self.new_terminal.restore_terminal().expect("Terminal Restore Failed");
    }


    fn process_keypresses(&mut self) -> Result<(), std::io::Error> {
        if let Event::Key(key) = event::read().unwrap(){
            match key.code {
                KeyCode::Char('q') => self.quit = true,
                KeyCode::F(1) => self.curr_scene = Scenes::MainList,
                KeyCode::F(2) => self.curr_scene = Scenes::AnimeSearch,
                _ => match &self.curr_scene{
                    Scenes::MainList => self.new_main_list.process_key(key.code),
                    Scenes::AnimeSearch=> self.new_anime_search.process_key(key.code),
                    _ => {}
                }
            }
        }

        Ok(())
    }

    fn setup(& mut self) {

        self.interface.authentication();
        self.interface.fetch_viewer().expect("ERROR: Failed to find Viewer for AniList Interface");

        self.new_main_list.set_anime_list(self.interface.fetch_anime_list(self.new_main_list.get_current_sort()));
        self.new_main_list.set_widget_strings();

    }
}

fn die(e: &std::io::Error) {
    OldTerminal::clear_screen();
    panic!("{}", e);
}
