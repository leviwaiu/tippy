use std::io::Stdout;
use crate::list_entry::{ListEntry, ListStatus};
use crate::scene::Displayable;

use tui::{
    backend::Backend,
    Frame,
    style::{Color, Style},
    widgets::{Row, Table, TableState},
};
use tui::layout::{Constraint, Direction, Layout, Rect};

use crossterm::event::KeyCode;
use tui::backend::CrosstermBackend;
use crate::anilist::interface::AniListInterface;

pub struct MainList {
    display_list: Vec<ListEntry>,

    status_bar: String,

    widget_table: Vec<Vec<String>>,
    widget_state: TableState,
    current_sort: ListStatus,

    _change_viewcount: Option<ListEntry>,
}

impl Displayable for MainList {
    fn widget(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>) {

        let mut fsize_mod = f.size();
        fsize_mod.height -= 1;

        let mut layout = Layout::default().direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(100),
            ]).split(fsize_mod);

        layout.push(Rect {
            x:0,
            y:fsize_mod.height,
            width: fsize_mod.width,
            height:1,
        });


        let mut table_vector = Vec::new();

        //Change Up this part

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
        match &self._change_viewcount {
            Some(item) => {
                interface.edit_anime_watchcount(item.clone()).expect("Error: CHANGE_EDIT");
                self._change_viewcount = None;
            }
            None => {},
        }
    }

}


impl MainList {
    pub fn default() -> Self {
        Self {
            display_list: Vec::new(),

            status_bar: String::from("Welcome to Tippy!"),
            widget_table: Vec::new(),
            widget_state: TableState::default(),
            current_sort: ListStatus::CURRENT,

            _change_viewcount:None,
        }
    }

    pub fn set_widget_strings(&mut self) {
        self.widget_table = Vec::new();
        for x in 0..self.display_list.len() {
            self.widget_table.push(self.create_string(x));
        }
    }

    fn create_string(&self, index: usize) -> Vec<String> {
        if index >= self.display_list.len() {
            return vec!["".to_string(),
                        "".to_string(),
                        "".to_string(),
                        "".to_string()];
        }
        let watchcount = format!("{}/{}",
                                 self.display_list[index].watched_count(),
                                 self.display_list[index].total_count());
        let output = vec!(
            self.display_list[index].title().to_string(),
            watchcount,
            self.display_list[index].score().to_string(),
            self.display_list[index].status().to_description()
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
        let current = &mut self.display_list[i];
        match key {
            KeyCode::Char('+') => {
                current.add_watched();
            }
            KeyCode::Char('-') => {
                current.remove_watched();
            }
            _ => {}
        }

        self._change_viewcount = Some(self.display_list[i].clone());
        self.set_widget_strings();

    }

    pub fn get_display_list(&self) -> Vec<ListEntry> {
        self.display_list.clone()
    }

    pub fn set_display_list(&mut self, list: Vec<ListEntry>) {
        self.display_list = list;
    }

    pub fn get_display_list_by_status(&mut self, full_list: Vec<ListEntry>) {
        self.display_list = Vec::new();
        for entry in full_list{
            if entry.status() == self.current_sort {
                self.display_list.push(entry.clone());
            }
        };
    }

    pub fn get_current_sort(&self) -> ListStatus {
        self.current_sort.clone()
    }
}