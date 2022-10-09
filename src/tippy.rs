use crate::terminal::{Position, OldTerminal};
use termion::event::Key;

use crate::anilist::interface::AniListInterface;
use crate::scene::{
    anime_search::AnimeSearch,
    mainlist::MainList,
    settings::SettingsScene,
    Scene,
    SceneTrait
};
use std::{io, rc::Rc};
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use crate::new_scene::main_list::MainList as NewMainList;
use crate::new_scene::{Displayable, Scenes};
use crate::new_terminal::TerminalInterface;

pub struct Tippy {
    terminal: OldTerminal,
    new_terminal: TerminalInterface,
    quit: bool,
    interface: AniListInterface,

    scene: Rc<Scene>,

    curr_scene: Scenes,
    new_main_list: NewMainList,

    main_list: Option<Rc<Scene>>,
    settings: Option<Rc<Scene>>,
    anime_search: Option<Rc<Scene>>,
}

impl Tippy {
    pub fn default() -> Self {

        let out = Self {

            terminal: OldTerminal::default().expect("Terminal Initialisation Failed"),
            new_terminal: TerminalInterface::default().expect("New Terminal Interface Initialisation Failed"),

            quit: false,
            interface: AniListInterface::default(),

            scene: Rc::new(Scene::MainList(MainList::default())),

            curr_scene: Scenes::MainList,
            new_main_list: NewMainList::default(),

            main_list: None,
            settings: Some(Rc::new(Scene::Settings(SettingsScene::default()))),
            anime_search: Some(Rc::new(Scene::AnimeSearch(AnimeSearch::default()))),
        };
        out
    }

    pub fn run(&mut self) {
        self.setup();

        // loop {
        //     if let Err(error) = self.process_screen_tick() {
        //         die(&error);
        //     }
        //     if self.quit {
        //         break;
        //     }
        //     if let Err(error) = self.process_keypresses() {
        //         die(&error);
        //     }
        //     Rc::<Scene>::get_mut(&mut self.scene)
        //         .unwrap()
        //         .connect_interface(&mut self.interface);
        // }

        loop {
            let main_list = &mut self.new_main_list;
            self.new_terminal.render_widget(|f| main_list.widget(f)).expect("TODO: panic message");
            if let Err(error) = self.process_keypresses() {
                die(&error);
            }
            if self.quit {
                break;
            }
        }

        self.new_terminal.restore_terminal().expect("Terminal Restore Failed");
    }

    fn process_screen_tick(&self) -> Result<(), std::io::Error> {
        OldTerminal::cursor_hide();
        OldTerminal::clear_screen();
        OldTerminal::cursor_position(&Position::default());
        if self.quit {
            OldTerminal::cursor_show();
            println!("Exiting...\r");
        } else {
            self.scene.show_view(&self.terminal);
        }
        OldTerminal::flush()
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
        // let pressed_key = OldTerminal::read_key()?;
        // let settingref = match self.settings.as_ref() {
        //     Some(s) => s,
        //     None => self.scene.as_ref(),
        // };
        // if let Scene::Settings(settings_scene) = settingref {
        //     let settings = settings_scene.get_settings();
        //     match pressed_key {
        //         Key::Char('q') => self.quit = true,
        //         Key::F(1) => self.change_scene(self.main_list.clone()),
        //         Key::F(2) => self.change_scene(self.anime_search.clone()),
        //         Key::F(8) => self.change_scene(self.settings.clone()),
        //         _ => Rc::<Scene>::get_mut(&mut self.scene).unwrap().process_key(
        //             pressed_key,
        //             &self.terminal,
        //             settings,
        //         ),
        //     }
        // }

        if let Event::Key(key) = event::read().unwrap(){
            match key.code {
                KeyCode::Char('q') => self.quit = true,
                _ => match &self.curr_scene{
                    MainList => self.new_main_list.process_key(key.code),
                    _ => {}
                }
            }
        }

        Ok(())
    }

    fn setup(&mut self) {

        self.interface.authentication();
        self.interface.fetch_viewer().expect("ERROR: Failed to find Viewer for AniList Interface");

        //Old Termion Based UI
        // self.terminal
        //     .put_into_raw()
        //     .expect("Terminal Initialisation Failed");
        //
        // if let Scene::MainList(main_list) = Rc::<Scene>::get_mut(&mut self.scene).unwrap() {
        //     main_list.set_anime_list(self.interface.fetch_anime_list(main_list.current_sort()));
        // }

        //New TUI (Testing)
        self.new_main_list.set_anime_list(self.interface.fetch_anime_list(self.new_main_list.get_current_sort()));
        self.new_main_list.set_widget_strings();

    }
}

fn die(e: &std::io::Error) {
    OldTerminal::clear_screen();
    panic!("{}", e);
}
