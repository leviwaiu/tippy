use crate::scene::SceneTrait;
use crate::terminal::{Terminal, Position};
use termion::event::Key;
use crate::scene::settings::Settings;
use crate::anilist_interface::AniListInterface;
use termion::color;
use std::io::stdout;
use termion::cursor::DetectCursorPos;

pub struct AnimeSearch {
    keyword: String,
    new_keyword: bool,
    entering: bool,
    enter_string: String,
    position: Position,
    selected: Position,
}

impl SceneTrait for AnimeSearch {
    fn show_view(&self, terminal: &Terminal) {
        Terminal::println_bgcolor(&*self.format_title(terminal), Box::new(color::Blue));
        self.show_searchbar(terminal);

        Terminal::cursor_position(&Position { x: 0, y: terminal.size().height as usize });
        Terminal::print_fgcolor(&*self.format_status_row(), Box::new(color::Blue));
        if self.entering {
            Terminal::cursor_show();
        }
    }

    fn format_status_row(&self) -> String {
        if self.entering {
            self.enter_string.clone()
        }
        else {
            String::from("Ready to Search")
        }
    }

    fn process_key(&mut self, key: Key, terminal: &Terminal, settings: Settings) {
        match key{
            Key::Up
            | Key::Down => self.move_cursor(key),
            Key::Char('\n') => self.enter_key(),
            Key::Backspace
            | Key::Delete
            | Key::Char(_) => {if self.entering {
                    self.input_text(key)
                } } ,
            _ => (),
        }
    }

    fn connect_interface(&mut self, interface: &AniListInterface) {
        if self.new_keyword {
            let connect = interface.search_anime(self.keyword.clone());
        }
    }
}

impl AnimeSearch {

    pub fn default() -> Self {
        Self {
            keyword: String::from(""),
            new_keyword: false,
            entering: false,
            enter_string: String::from(""),
            position: Position::default(),
            selected: Position::default(),
        }
    }

    fn format_title(&self, terminal: &Terminal) -> String{
        let width = terminal.size().width as usize;
        let str = "Search For Anime";

        format!("{}{}", str , " ".repeat(width - str.len()))
    }

    fn input_text(&mut self, key:Key){
        match key {
            Key::Delete => {}, //placeholder
            Key::Backspace => {let _ = self.enter_string.pop(); },
            Key::Char(char) => self.enter_string.push(char),
            _ => {}
        };
    }

    fn move_cursor(&mut self, key:Key){
        let Position {x, mut y} = self.selected;
        let list_length = 3 as usize;

        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down =>
                if y < list_length.saturating_sub(1) {
                    y = y.saturating_add(1);
                },
            _ => ()
        }

        self.selected = Position {x, y}
    }

    fn show_searchbar(&self, terminal: &Terminal){
        let width = terminal.size().width as usize;
        let size = width / 5;
        Terminal::print_fgcolor("Search Term",Box::new(termion::color::Blue));
        let curr_y = stdout().cursor_pos().unwrap().1 as usize;
        let desired_position = Position{x: size, y: curr_y.saturating_sub(1)};
        Terminal::cursor_position(&desired_position);

        let padlength = width - ( size + self.keyword.len());
        let keyword_display = format!("{}{}", self.keyword, " ".repeat(padlength));

        Terminal::println_bgcolor(keyword_display.as_str(), Box::new(termion::color::White));
        Terminal::println_fgcolor("Advanced Options", Box::new(termion::color::Blue));
        Terminal::println_fgcolor("Search", Box::new(termion::color::Blue));
        Terminal::println_fgcolor("Reset Search", Box::new(termion::color::Blue));
    }

    fn enter_key(&mut self){
        if self.entering {
            self.keyword = self.enter_string.clone();
            self.entering = false;
            self.enter_string = String::from("");
        }
        else {
            self.entering = true;
        }
    }
}