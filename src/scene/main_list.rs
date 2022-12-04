use std::io::Stdout;
use crate::list_entry::{ListEntry, ListStatus};
use crate::scene::Displayable;

use tui::{
    backend::Backend,
    Frame,
    style::{Color, Style},
    widgets::{Row, Table, TableState},
};
use tui::layout::{Constraint, Direction, Layout};

use crossterm::event::KeyCode;
use tui::backend::CrosstermBackend;
use crate::anilist::interface::AniListInterface;

pub struct MainList {
    anime_list: Vec<ListEntry>,
    filtered_list: Vec<ListEntry>,

    status_bar: String,

    widget_table: Vec<Vec<String>>,
    widget_state: TableState,
    current_sort: ListStatus,

    _change: Option<ListEntry>,
}

impl Displayable for MainList {
    fn widget(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>) {
        let layout = Layout::default().direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(99),
                Constraint::Length(1),
            ]).split(f.size());

        let mut table_vector = Vec::new();
        for x in 0..self.widget_table.len() {
            table_vector.push(Row::new(self.widget_table[x].clone()));
        }
        let table_widget = Table::new(table_vector)
            .header(
                Row::new(vec!["Name", "Progress", "Score", "Type"])
                    .style(Style::default().bg(Color::Blue).fg(Color::White))
            )
            .widths(&[Constraint::Percentage(60),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10)
            ])
            .highlight_style(
                Style::default().bg(Color::Black).fg(Color::White)
            );

        let status_bar = Table::new([
            Row::new([self.status_bar.clone()]).style(Style::default().fg(Color::Blue))
        ]).widths(&[Constraint::Percentage(100)]);

        self.set_widget_strings();
        f.render_stateful_widget(table_widget, layout[0], &mut self.widget_state.clone());
        f.render_widget(status_bar, layout[1]);
    }

    fn process_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up => self.move_prev(),
            KeyCode::Down => self.move_next(),
            KeyCode::Char('+') | KeyCode::Char('-') => self.edit_watchcount(key),
            _ => {}
        }
    }

    fn connect_interface(&mut self, interface: &AniListInterface) {
        match &self._change {
            Some(item) => {
                interface.edit_anime_watchcount(item.clone()).expect("Error: CHANGE_EDIT");
                self._change = None;
            }
            None => {},
        }
    }

}


impl MainList {
    pub fn default() -> Self {
        Self {
            anime_list: Vec::new(),

            filtered_list: Vec::new(),

            status_bar: String::from("Welcome to Tippy!"),
            widget_table: Vec::new(),
            widget_state: TableState::default(),
            current_sort: ListStatus::CURRENT,

            _change:None,
        }
    }

    pub fn set_widget_strings(&mut self) {
        self.widget_table = Vec::new();
        for x in 0..self.anime_list.len() {
            self.widget_table.push(self.create_string(x));
        }
    }

    fn create_string(&self, index: usize) -> Vec<String> {
        if index >= self.anime_list.len() {
            return vec!["".to_string(),
                        "".to_string(),
                        "".to_string(),
                        "".to_string()];
        }
        let watchcount = format!("{}/{}",
                                 self.anime_list[index].watched_count(),
                                 self.anime_list[index].total_count());
        let output = vec!(
            self.anime_list[index].title().to_string(),
            watchcount,
            self.anime_list[index].score().to_string(),
            self.anime_list[index].status().to_description()
        );

        output
    }

    fn move_next(&mut self) {
        let i = match self.widget_state.selected() {
            Some(x) => {
                if x < self.widget_table.len() - 1 {
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

    fn edit_watchcount(&mut self, key: KeyCode) {
        let i = self.widget_state.selected().expect("Error: WATCH_NONE");
        let current = &mut self.anime_list[i];
        match key {
            KeyCode::Char('+') => {
                current.add_watched();
            }
            KeyCode::Char('-') => {
                current.remove_watched();
            }
            _ => {}
        }

        self._change = Some(self.anime_list[i].clone());
        self.set_widget_strings();

    }

    pub fn get_anime_list(&self) -> Vec<ListEntry> {
        self.anime_list.clone()
    }

    pub fn set_anime_list(&mut self, list: Vec<ListEntry>) {
        self.anime_list = list;
    }

    pub fn get_current_sort(&self) -> ListStatus {
        self.current_sort.clone()
    }
}