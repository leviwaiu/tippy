use crate::anilist::interface::AniListInterface;

use crossterm::event;
use crossterm::event::{Event, KeyCode};
use futures::future::err;
use crate::scene::{main_list::MainList, anime_search::AnimeSearch, Displayable, Scenes};
use crate::terminal::TerminalInterface;

pub struct Tippy {
    new_terminal: TerminalInterface,
    quit: bool,
    interface: AniListInterface,

    curr_scene: Scenes,
    main_list: MainList,
    anime_search: AnimeSearch,
}

impl Tippy {
    pub fn default() -> Self {

        let out = Self {

            new_terminal: TerminalInterface::default().expect("New Terminal Interface Initialisation Failed"),

            quit: false,
            interface: AniListInterface::default(),

            curr_scene: Scenes::MainList,
            main_list: MainList::default(),
            anime_search: AnimeSearch::default(),

        };
        out
    }

    pub fn run(&mut self) {
        self.setup();

        loop {
            let display_scene: Box<&mut dyn Displayable> = match &self.curr_scene {
                Scenes::MainList => Box::new(&mut self.main_list),
                Scenes::AnimeSearch => Box::new(&mut self.anime_search),
            };

            self.new_terminal.render_widget(|f| display_scene.widget(f)).expect("TODO: panic message");

            let key_event = event::read();

            if let Err(error) = &key_event {
                die(error);
            };

            let key = key_event.unwrap();
            if let Event::Key(k) = key {
                match k.code {
                    KeyCode::Char('q') => self.quit = true,
                    KeyCode::F(1) => self.curr_scene = Scenes::MainList,
                    KeyCode::F(2) => self.curr_scene = Scenes::AnimeSearch,
                    _ => display_scene.process_key(k.code),
                }
            }

            if self.quit {
                break;
            }

            display_scene.connect_interface(&self.interface);

        }

        self.new_terminal.restore_terminal().expect("Terminal Restore Failed");
    }


    fn setup(& mut self) {
        self.interface.authentication();
        self.interface.fetch_viewer().expect("ERROR: Failed to find Viewer for AniList Interface");

        self.interface.fetch_full_anime_list();
        self.main_list.get_display_list_by_status(self.interface.get_main_list());
        self.main_list.set_display_string_table();
    }
}

fn die(e: &std::io::Error) {
    panic!("{}", e);
}
