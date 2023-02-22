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
use tui::layout::Rect;
use tui::widgets::List;
use crate::anilist::interface::AniListInterface;
use crate::list_entry::ListStatus;

use crate::scene::Displayable;
use crate::search_entry::AnimeSearchEntry;

pub struct AnimeSearch {
    keyword: String,
    widget_state: TableState,
    result_display: Vec<AnimeSearchEntry>,
    entering: bool,
    search_ready: bool,
}

impl Displayable for AnimeSearch {
    fn widget(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>){

        let mut fsize_mod = f.size();
        fsize_mod.height -= 1;

        let mut layout = Layout::default().direction(Direction::Vertical)
            .constraints([
                Constraint::Min(6),
                Constraint::Percentage(99),
            ]).split(fsize_mod);

        layout.push(Rect {
            x:0,
            y: fsize_mod.height,
            width: fsize_mod.width,
            height:1,
        });

        let search_field = Table::new([
            Row::new(["Search String:", self.keyword.as_str()]),
            Row::new(["Advanced Search", ""]),
            Row::new(["",""]),
            Row::new(["Search", ""]),
            Row::new(["Reset", ""])
        ]).style(Style::default().fg(Color::Blue)).widths(&[Constraint::Min(20),
                                                          Constraint::Percentage(80)])
        .highlight_style(
            Style::default().bg(Color::Black).fg(Color::White)
        );

        let mut result_table = Vec::new();
        for x in &self.result_display {
            result_table.push(Row::new(x.make_vec()));
        }
        let search_results = Table::new(result_table)
            .widths(&[Constraint::Percentage(40),
            Constraint::Percentage(20),
            Constraint::Percentage(15),
            Constraint::Percentage(15)
        ]);

        let status_bar = Table::new([
            Row::new(["Test Thing Only here"]).style(Style::default().fg(Color::Blue))
        ]).widths(&[Constraint::Percentage(100)]);

        f.render_stateful_widget(search_field, layout[0], &mut self.widget_state.clone());
        f.render_widget(search_results, layout[1]);
        f.render_widget(status_bar, layout[2]);
    }

    fn process_key(&mut self, key: KeyCode) {

        match key {
            KeyCode::Enter => self.press_enter(),
            KeyCode::Up => self.move_prev(),
            KeyCode::Down => self.move_next(),
            KeyCode::Char(x) => if self.entering {
                self.keyword = self.keyword.clone() + &*x.to_string();
            }
            KeyCode::Backspace => if self.entering {
                self.keyword.pop();
            }
            _ => {},
        }
    }

    fn connect_interface(&mut self, interface: &AniListInterface) {
        if self.search_ready {
            self.result_display = interface.search_anime(self.keyword.clone()).expect("TODO: Something went wrong here");
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
                if x == 1{
                    x + 2
                } else if x < 4 {
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
                if i == 3 {
                    i - 2
                } else if i > 0{
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
