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
use crate::anilist::interface::AniListInterface;

use crate::scene::Displayable;

pub struct AnimeSearch {
    keyword: String,
    widget_state: TableState,

}

impl Displayable for AnimeSearch {
    fn widget(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>){

        let layout = Layout::default().direction(Direction::Vertical)
            .constraints([
                Constraint::Length(6),
                Constraint::Percentage(99),
                Constraint::Length(1),
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
            KeyCode::Enter => {},
            _ => {},
        }
    }

    fn connect_interface(&mut self, interface: &AniListInterface) {
        todo!()
    }
}

impl AnimeSearch {
    pub fn default()-> Self {
        Self{
            keyword:String::new(),
            widget_state: TableState::default(),
        }
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