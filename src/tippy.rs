use crate::terminal::{Position, Terminal};
use termion::event::Key;

use crate::anilist::interface::AniListInterface;
use crate::scene::anime_search::AnimeSearch;
use crate::scene::mainlist::MainList;
use crate::scene::settings::SettingsScene;
use crate::scene::{Scene, SceneTrait};
use std::rc::Rc;

pub struct Tippy {
    terminal: Terminal,
    quit: bool,
    interface: AniListInterface,

    scene: Rc<Scene>,

    main_list: Option<Rc<Scene>>,
    settings: Option<Rc<Scene>>,
    anime_search: Option<Rc<Scene>>,
}

impl Tippy {
    pub fn default() -> Self {
        let out = Self {
            terminal: Terminal::default().expect("Terminal Initialisation Failed"),
            quit: false,
            interface: AniListInterface::default(),

            scene: Rc::new(Scene::MainList(MainList::default())),

            main_list: None,
            settings: Some(Rc::new(Scene::Settings(SettingsScene::default()))),
            anime_search: Some(Rc::new(Scene::AnimeSearch(AnimeSearch::default()))),
        };
        out
    }

    pub fn run(&mut self) {
        self.setup();

        loop {
            if let Err(error) = self.process_screen_tick() {
                die(&error);
            }
            if self.quit {
                break;
            }
            if let Err(error) = self.process_keypresses() {
                die(&error);
            }
            Rc::<Scene>::get_mut(&mut self.scene)
                .unwrap()
                .connect_interface(&mut self.interface);
        }
    }

    fn process_screen_tick(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::clear_screen();
        Terminal::cursor_position(&Position::default());
        if self.quit {
            Terminal::cursor_show();
            println!("Exiting...\r");
        } else {
            self.scene.show_view(&self.terminal);
        }
        Terminal::flush()
    }

    fn change_scene(&mut self, scene: Option<Rc<Scene>>) {
        let scene_unwrap: Rc<Scene>;

        match scene {
            Some(x) => scene_unwrap = Rc::clone(&x),
            None => return,
        }

        match &*self.scene {
            Scene::Settings(_) => self.settings = Some(Rc::clone(&self.scene)),
            Scene::MainList(_) => self.main_list = Some(Rc::clone(&self.scene)),
            Scene::AnimeSearch(_) => self.anime_search = Some(Rc::clone(&self.scene)),
        }

        self.scene = Rc::clone(&scene_unwrap);

        match *scene_unwrap {
            Scene::Settings(_) => self.settings = None,
            Scene::MainList(_) => self.main_list = None,
            Scene::AnimeSearch(_) => self.anime_search = None,
        }
    }

    fn process_keypresses(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        let settingref = match self.settings.as_ref() {
            Some(s) => s,
            None => self.scene.as_ref(),
        };
        if let Scene::Settings(settings_scene) = settingref {
            let settings = settings_scene.get_settings();
            match pressed_key {
                Key::Char('q') => self.quit = true,
                Key::F(1) => self.change_scene(self.main_list.clone()),
                Key::F(2) => self.change_scene(self.anime_search.clone()),
                Key::F(8) => self.change_scene(self.settings.clone()),
                _ => Rc::<Scene>::get_mut(&mut self.scene).unwrap().process_key(
                    pressed_key,
                    &self.terminal,
                    settings,
                ),
            }
        }

        Ok(())
    }

    fn setup(&mut self) {
        self.terminal
            .put_into_raw()
            .expect("Terminal Initialisation Failed");

        self.interface.authentication();
        self.interface.fetch_viewer();
        if let Scene::MainList(main_list) = Rc::<Scene>::get_mut(&mut self.scene).unwrap() {
            main_list.set_anime_list(self.interface.fetch_anime_list(main_list.current_sort()));
        }

        //REMOVE WHEN NEEDED
        //self.terminal.debug_size_override();
    }
}

fn die(e: &std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
