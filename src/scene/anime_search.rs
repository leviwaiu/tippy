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
    result_display: Option<Vec<AnimeSearchEntry>>,
    result_state: TableState,
    entering: bool,
    search_ready: bool,

    selected_anime: Option<usize>,

    toolbar_text: String,
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
            Row::new(["Search by Title", self.keyword.as_str()]),
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
        if let Some(dis_list) = &self.result_display {
            for x in dis_list {
                result_table.push(Row::new(x.make_vec()));
            }
        }
        let search_results = Table::new(result_table)
            .widths(&[Constraint::Percentage(40),
            Constraint::Percentage(20),
            Constraint::Percentage(15),
            Constraint::Percentage(15)
        ]).highlight_style(
            Style::default().bg(Color::Black).fg(Color::White)
        );;

        let status_bar = Table::new([
            Row::new([self.toolbar_text.clone()]).style(Style::default().fg(Color::Blue))
        ]).widths(&[Constraint::Percentage(100)]);

        f.render_stateful_widget(search_field, layout[0], &mut self.widget_state.clone());
        f.render_stateful_widget(search_results, layout[1], &mut self.result_state.clone());
        f.render_widget(status_bar, layout[2]);
    }

    fn process_key(&mut self, key: KeyCode) {

        match key {
            KeyCode::Enter => self.press_enter(),
            KeyCode::Up => self.move_prev(),
            KeyCode::Down => self.move_next(),
            KeyCode::Char(x) => if self.entering {
                self.toolbar_text = self.toolbar_text.clone() + &*x.to_string();
            }
            KeyCode::Backspace => if self.entering {
                self.toolbar_text.pop();
            }
            _ => {},
        }
    }

    fn connect_interface(&mut self, interface: &AniListInterface) {
        if self.search_ready {
            self.result_display =
                Some(interface.search_anime(self.keyword.clone())
                    .expect("TODO: Something went wrong here"));
            self.result_state.select(Some(0));
            self.widget_state.select(None);
            self.search_ready = false;
        }

        if let Some(media_id) = self.selected_anime {
            interface.get_anime_details(media_id).expect("TODO: panic message");
        }
    }
}

impl AnimeSearch {
    pub fn default()-> Self {
        Self{
            keyword:String::new(),
            widget_state: TableState::default(),
            result_display: None,
            result_state: TableState::default(),
            entering: false,
            search_ready: false,

            selected_anime: None,

            toolbar_text: String::from("Search Function"),
        }
    }

    fn press_enter(&mut self) {

        match self.widget_state.selected() {
            Some(0) => if !self.entering {
                self.entering = true;
                self.toolbar_text = String::new();
            }
            else {
                self.entering = false;
                self.keyword = self.toolbar_text.clone();
                self.toolbar_text = String::from("Search Function");
            }
            ,
            Some(1) => {}, // Advanced Search,
            Some(2) => {}, //
            Some(3) => self.search_ready = !self.search_ready, //Search,
            Some(4) => self.reset(), //Reset,
            None => self.enter_on_result(), // General List Selection
            _ => {}
        }
    }

    fn enter_on_result(&mut self) {
        if let Some(x) = &self.result_display {
            let result = &x[self.result_state.selected().unwrap()];
            self.selected_anime = Some(result.get_id());
        }


    }

    fn reset(&mut self) {
        self.keyword = String::new();
        self.widget_state.select(Some(0));
        self.result_state.select(None);
        self.result_display = None;
        self.entering = false;
        self.search_ready = false;
    }

    fn move_next(&mut self) {
        if self.entering {
            return;
        }

        if let Some(list) = &self.result_display {
            let i = match self.result_state.selected() {
                Some(i) => {
                    if i + 1 == list.len() {
                        i
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            self.result_state.select(Some(i));
            self.widget_state.select(None);
        } else {
            let i = match self.widget_state.selected() {
                Some(i) => {
                    if i == 1 {
                        i + 2
                    } else if i < 4 {
                        i + 1
                    } else {
                        i
                    }
                }
                None => 0,
            };
            self.widget_state.select(Some(i));
        }
    }

    fn move_prev(&mut self) {
        if self.entering {
            return;
        }

        if let Some(_) = &self.result_display {
            let i = match self.result_state.selected() {
                Some(i) => {
                    if i == 0 {
                        None
                    } else {
                        Some(i - 1)
                    }
                }
                None => None,
            };
            let j = match self.widget_state.selected() {
                Some(i) => Some(i),
                None => if i == None {
                    Some(4)
                } else {
                    None
                },
            };

            self.result_state.select(i);
            self.widget_state.select(j);
        } else {
            let i = match self.widget_state.selected() {
                Some(i) => {
                    if i == 3 {
                        i - 2
                    } else if i > 0 {
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

}
