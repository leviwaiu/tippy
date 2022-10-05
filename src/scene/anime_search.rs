use std::cmp::min;
use crate::anilist::interface::AniListInterface;
use crate::scene::settings::Settings;
use crate::scene::SceneTrait;
use crate::terminal::{Position, OldTerminal};
use std::io::stdout;
use termion::color;
use termion::cursor::DetectCursorPos;
use termion::event::Key;

struct SearchResult {
    id: usize,
    title: String,
    media_type: String,
    added: bool,
}

pub struct AnimeSearch {
    keyword: String,
    search_commence: bool,
    entering: bool,
    enter_string: String,
    position: Position,
    selected: Position,
    offset: Position,

    search_results: Vec<SearchResult>,
}

const LIST_LENGTH :usize = 4;
const LIST_DISPLAY_LENGTH:usize = 6;

impl SceneTrait for AnimeSearch {
    fn show_view(&self, terminal: &OldTerminal) {
        OldTerminal::println_bgcolor(&*self.format_title(terminal), Box::new(color::Blue));
        self.show_searchbar(terminal);
        OldTerminal::println_fgcolor(
            &*"─".repeat(terminal.size().width as usize),
            Box::new(color::Blue),
        );
        self.show_results(terminal);

        OldTerminal::cursor_position(&Position {
            x: 0,
            y: terminal.size().height as usize,
        });
        OldTerminal::print_fgcolor(&*self.format_status_row(), Box::new(color::Blue));
        if self.entering {
            OldTerminal::cursor_show();
        }
    }

    fn format_status_row(&self) -> String {
        if self.entering {
            self.enter_string.clone()
        } else {
            format!("{} {} {}","Ready to Search", self.offset.y, self.selected.y)
        }
    }

    fn process_key(&mut self, key: Key, terminal: &OldTerminal, settings: Settings) {
        match key {
            Key::Up | Key::Down => self.move_cursor(key, terminal),
            Key::Char('\n') => self.enter_key(),
            Key::Backspace | Key::Delete | Key::Char(_) => {
                if self.entering {
                    self.input_text(key)
                }
            }
            _ => (),
        }
        self.scroll(terminal);
    }

    fn connect_interface(&mut self, interface: &mut AniListInterface) {
        if self.search_commence {
            let mut output = Vec::default();
            let connect = interface.search_anime(self.keyword.clone()).unwrap();
            //print!("{}", connect.to_string());
            let res = connect["data"]["Page"]["media"].as_array().unwrap();
            let main_list = interface.get_main_list();
            let mut id_list : Vec<u64> = Vec::new();

            for entry in main_list {
                id_list.push(entry.media_id() as u64);
                print!("[{}],", entry.id());
            }

            for value in res {
                output.push(SearchResult {
                    id: value["id"].as_u64().unwrap() as usize,
                    title: String::from(match value["title"]["native"].as_str() {
                        Some(val) => val,
                        None => value["title"]["romaji"].as_str().unwrap()
                    }),
                    media_type: String::from(value["format"].as_str().unwrap()),
                    added: id_list.contains(&value["id"].as_u64().unwrap()),
                });
                print!("{}//", value["id"]);
            }
            self.search_results = output;
            self.search_commence = false;
            self.selected.y = 4;
        }
    }
}

impl AnimeSearch {
    pub fn default() -> Self {
        Self {
            keyword: String::from(""),
            search_commence: false,
            entering: false,
            enter_string: String::from(""),
            position: Position::default(),
            selected: Position::default(),
            offset: Position::default(),

            search_results: Vec::default(),
        }
    }

    fn format_title(&self, terminal: &OldTerminal) -> String {
        let width = terminal.size().width as usize;
        let str = "Search For Anime";

        format!("{}{}", str, " ".repeat(width - str.len()))
    }

    fn input_text(&mut self, key: Key) {
        match key {
            Key::Delete => {} //placeholder
            Key::Backspace => {
                let _ = self.enter_string.pop();
            }
            Key::Char(char) => self.enter_string.push(char),
            _ => {}
        };
    }

    fn move_cursor(&mut self, key: Key, terminal:&OldTerminal) {
        let Position { x, mut y } = self.selected;
        let list_length = 4 + self.search_results.len();

        let mut cursor_ceiling = 0;
        if self.search_results.len() > 0 {
            cursor_ceiling = 3;
        }

        match key {
            Key::Up => {
                if y > cursor_ceiling {
                    y = y.saturating_sub(1)
                }
            }
            Key::Down => {
                if y < list_length.saturating_sub(1) {
                    y = y.saturating_add(1);
                }
            }
            _ => (),
        }

        self.selected = Position { x, y }
    }

    fn scroll(&mut self, terminal: &OldTerminal){
        let Position { x: _, y } = self.selected;
        let _width = terminal.size().width as usize;
        let height = terminal.size().height as usize;
        let mut offset = &mut self.offset;

        if y.saturating_sub(4) <= offset.y {
            offset.y = y.saturating_sub(4);
        } else if y >= offset.y.saturating_add(height - 3) {
            offset.y = y.saturating_sub(height - 3).saturating_add(1);
        }
    }

    fn show_searchbar(&self, terminal: &OldTerminal) {
        let width = terminal.size().width as usize;
        let size = width / 5;
        OldTerminal::print_fgcolor("Search Term", Box::new(termion::color::Blue));
        let curr_y = stdout().cursor_pos().unwrap().1 as usize;
        let desired_position = Position {
            x: size,
            y: curr_y.saturating_sub(1),
        };
        OldTerminal::cursor_position(&desired_position);

        let padlength = width - (size + self.keyword.len());
        let keyword_display = format!("{}{}", self.keyword, " ".repeat(padlength));
        //Terminal::println_bgcolor(keyword_display.as_str(), Box::new(termion::color::White));

        let buttons = [
            keyword_display.as_str(),
            "Advanced Options",
            "Search",
            "Reset Search",
        ];

        for row_no in 0..buttons.len() {
            if self.selected.y == row_no {
                if row_no == 0 {
                    OldTerminal::println_bgcolor(
                        keyword_display.as_str(),
                        Box::new(termion::color::White),
                    );
                } else {
                    OldTerminal::println_color(
                        &*buttons[row_no],
                        Box::new(termion::color::LightWhite),
                        Box::new(termion::color::Blue),
                    );
                }
            } else {
                if row_no == 0 {
                    print!("{}", buttons[row_no]);
                } else {
                    OldTerminal::println_fgcolor(&*buttons[row_no], Box::new(termion::color::Blue));
                }
            }
        }
    }

    fn show_results(&self, terminal:&OldTerminal) {
        let height = terminal.size().height as usize;
        for number in self.offset.y..self.search_results.len() {
            if (number - self.offset.y) > (height - LIST_DISPLAY_LENGTH - 2){
                return;
            }
            let result = self.search_results.get(number).unwrap();
            if self.selected.y == (number + 4) {
                OldTerminal::println_bgcolor(
                    &*format!("{}         {}   {}\r", result.title, result.media_type, result.added),
                    Box::new(termion::color::Blue))
            }
            else {
                println!("{}         {}   {}\r", result.title, result.media_type, result.added);
            }
        }
    }

    fn enter_key(&mut self) {
        match self.selected.y {
            0 => {
                if self.entering {
                    self.keyword = self.enter_string.clone();
                    self.entering = false;
                    self.enter_string = String::from("");
                } else {
                    self.entering = true;
                }
            }
            2 => self.search_commence = true,
            3 => {
                self.keyword = String::from("");
                self.search_results = Vec::new();
                self.selected.y = 0;
            },
            _ => {}
        }
    }
}
