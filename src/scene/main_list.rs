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
use tui::widgets::{Block, Borders, List};
use crate::anilist::interface::AniListInterface;
use crate::list_entry::ListStatus::PAUSED;
use crate::scene::main_list::CurrentState::{NORMAL, SORTING};

pub struct MainList {
    display_list: Vec<ListEntry>,
    refresh_flag: bool,

    status_bar: String,

    display_string_table: Vec<Vec<String>>,
    display_status: TableState,
    side_table_status: TableState,
    current_sort: ListStatus,

    current_state: CurrentState,


    _change_viewcount: Option<ListEntry>,
}

#[derive(PartialEq)]
enum CurrentState {
    NORMAL,
    SORTING,
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
        self.set_display_string_table();
        //Change Up this part

        for x in 0..self.display_string_table.len() {
            table_vector.push(Row::new(self.display_string_table[x].clone()));
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


        if self.current_state == CurrentState::SORTING {
            let mut hori_layout = Layout::default().direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(70),
                    Constraint::Percentage(30),
                ]).split(layout[0]);

            let mut list_sort_row = Vec::new();

            for x in ListStatus::create_vec(){
                list_sort_row.push(Row::new([x.to_description()]));
            }

            let status_sort_table = Table::new(list_sort_row)
                .block(Block::default().title("Sort by Status:")
                    .borders(Borders::LEFT)
                    .border_style(Style::default().fg(Color::Blue)))
                .widths(&[Constraint::Percentage(100)])
                .highlight_style(
                    Style::default().bg(Color::Blue).fg(Color::White)
                );

            f.render_stateful_widget(table_widget, hori_layout[0], &mut self.display_status.clone());
            f.render_stateful_widget(status_sort_table, hori_layout[1], &mut self.side_table_status.clone());
        }
        else {
            f.render_stateful_widget(table_widget, layout[0], &mut self.display_status.clone());
        }

        let status_bar = Table::new([
            Row::new([self.status_bar.clone()]).style(Style::default().fg(Color::Blue))
        ]).widths(&[Constraint::Percentage(100)]);

        f.render_widget(status_bar, layout[1]);
    }

    fn process_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up => self.move_prev(),
            KeyCode::Down => self.move_next(),
            KeyCode::Enter => self.press_enter(),
            KeyCode::Char('s') => self.toggle_status(),
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
        if self.refresh_flag {
            self.get_display_list_by_status(interface.get_main_list());
            self.refresh_flag = false;
        }
    }

}


impl MainList {
    pub fn default() -> Self {
        Self {
            display_list: Vec::new(),
            refresh_flag: false,

            status_bar: String::from("Welcome to Tippy!"),
            display_string_table: Vec::new(),
            display_status: TableState::default(),
            side_table_status: TableState::default(),
            current_sort: ListStatus::CURRENT,

            current_state: NORMAL,

            _change_viewcount:None,
        }
    }

    pub fn set_display_string_table(&mut self) {
        self.display_string_table = Vec::new();
        for x in 0..self.display_list.len() {
            self.display_string_table.push(self.create_string(x));
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

    fn press_enter(&mut self){
        match self.current_state {
            NORMAL => todo!(),
            SORTING => {
                let list = ListStatus::create_vec();
                match self.side_table_status.selected(){
                    Some(i) => self.current_sort = list[i].clone(),
                    None => {}
                };
                self.refresh_flag = true;
                self.toggle_status();
            }
        }
    }

    fn move_next(&mut self) {
        match self.current_state {
            NORMAL => {
                let i = match self.display_status.selected() {
                    Some(x) => {
                        if x < self.display_string_table.len() - 1 {
                            x + 1
                        } else {
                            x
                        }
                    }
                    None => 0,
                };
                self.display_status.select(Some(i));
            }
            SORTING => {
                let i = match self.side_table_status.selected() {
                    Some(i) => {
                        if i < 5 {
                            i + 1
                        } else {
                            i
                        }
                    }
                    None => 0,
                };
                self.side_table_status.select(Some(i));
            }

        }

    }

    fn move_prev(&mut self) {
        match self.current_state {
            NORMAL => {
                let i = match self.display_status.selected() {
                    Some(i) => {
                        if i > 0 {
                            i - 1
                        } else {
                            i
                        }
                    }
                    None => 0,
                };
                self.display_status.select(Some(i));
            }
            SORTING => {
                let i = match self.side_table_status.selected() {
                    Some(i) => {
                        if i > 0 {
                            i - 1
                        } else {
                            i
                        }
                    }
                    None => 0,
                };
                self.side_table_status.select(Some(i));
            }
        }
    }

    fn edit_watchcount(&mut self, key: KeyCode) {
        let i = self.display_status.selected().expect("Error: WATCH_NONE");
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
        self.set_display_string_table();

    }

    fn toggle_status(&mut self) {
        if self.current_state == NORMAL {
            self.current_state = SORTING;
        }
        else {
            self.current_state = NORMAL;
        }
    }

    pub fn get_display_list_by_status(&mut self, full_list: Vec<ListEntry>) {
        self.display_list = Vec::new();
        for entry in full_list{
            if entry.status() == self.current_sort {
                self.display_list.push(entry.clone());
            }
        };
    }

}