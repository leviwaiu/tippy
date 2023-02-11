use std::io::Stdout;
use crossterm::event::KeyCode;
use tui::{
    Frame,
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Row, Table, TableState},
};
use tui::backend::CrosstermBackend;
use tui::widgets::List;
use crate::anilist::interface::AniListInterface;
use crate::list_entry::ListStatus;

use crate::scene::Displayable;

struct SearchEntry {
    name: String,
    id: String,
    added: ListStatus
}

pub struct AnimeSearch {
    keyword: String,
    widget_state: TableState,
    result_display: Vec<SearchEntry>,
    entering: bool,
    search_ready: bool,
}

impl Displayable for AnimeSearch {
    fn widget(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>){

        let layout = Layout::default().direction(Direction::Vertical)
            .constraints([
                Constraint::Length(6),
                Constraint::Percentage(99),
                Constraint::Min(1),
            ]).split(f.size());

        let search_field = Table::new([
            Row::new(["Search String:", self.keyword.as_str()]),
            Row::new(["Advanced Search:", ""]),
            Row::new(["",""]),
            Row::new(["Search", ""]),
            Row::new(["Reset", ""])
        ]).style(Style::default().fg(Color::Blue)).widths(&[Constraint::Percentage(100)]);

        let status_bar = Table::new([
            Row::new(["Test Thing Only here"]).style(Style::default().fg(Color::Blue))
        ]).widths(&[Constraint::Percentage(100)]);

        f.render_widget(search_field, layout[0]);
    }

    fn process_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Enter => self.press_enter(),
            _ => {},
        }
    }

    fn connect_interface(&mut self, interface: &AniListInterface) {
        if self.search_ready {
            let result = interface.search_anime(self.keyword.clone()).expect("TODO: Something went wrong here");
        }
    }
}

impl AnimeSearch {
    pub fn default()-> Self {
        Self{
            keyword:String::new(),
            widget_state: TableState::default(),
            result_display: Vec::new(),
            entering: false,
            search_ready: false,
        }
    }

    fn press_enter(&mut self) {
        match self.widget_state.selected() {
            Some(0) => self.entering = !self.entering,
            Some(1) => {}, // Advance Search,
            Some(2) => {}, //
            Some(3) => self.search_ready = !self.search_ready, //Search,
            Some(4) => self.reset(), //Reset,
            _ => {} // General List Selection
        }
    }

    fn reset(&mut self) {
        self.keyword = String::new();
        self.widget_state.select(Some(0));
        self.entering = false;
        self.search_ready = false;
    }

    fn move_next(&mut self) {
        let i = match self.widget_state.selected() {
            Some(x) => {
                if x < 4 {
                    x + 1
                } else {
                    x
                }
            }
            None => 0,
        };
        self.widget_state.select(Some(i));
    }

    fn move_prev(&mut self) {
        let i = match self.widget_state.selected() {
            Some(i) => {
                if i > 0 {
                    i - 1
                } else {
                    i
                }
            }
            None => 0,
        };
        self.widget_state.select(Some(i));
    }

}
