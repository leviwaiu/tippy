
use crossterm::event::KeyCode;
use tui::{
    Frame,
    backend::{Backend, CrosstermBackend},
    widgets::{Widget, Block, Borders},
};
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Row, Table, TableState};

use crate::new_scene::Displayable;

pub struct AnimeSearch {
    keyword: String,
    widget_state: TableState,

}

impl Displayable for AnimeSearch {
    fn widget<B:Backend>(&mut self, f: &mut Frame<B>){

        let layout = Layout::default().direction(Direction::Vertical)
            .constraints([
                Constraint::Length(6),
                Constraint::Percentage(100),
            ]).split(f.size());

        let search_field = Table::new([
            Row::new(["Search String:", self.keyword.as_str()]),
            Row::new(["",""]),
            Row::new(["Search", ""]),
            Row::new(["Reset", ""])
        ]).style(Style::default().fg(Color::Blue)).widths(&[Constraint::Percentage(100)]);

        f.render_widget(search_field, layout[0]);
    }

    fn process_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Enter => {},
            _ => {},
        }
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