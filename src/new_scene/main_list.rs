use crate::list_entry::{ListEntry, ListStatus};
use crate::new_scene::NewSceneTrait;

use tui::{
    backend::Backend,
    Frame,
};
use tui::layout::Constraint;
use tui::style::{Modifier, Style};
use tui::widgets::{Row, Table, TableState};

pub struct MainList {
    anime_list: Vec<ListEntry>,

    widget_table:Vec<Vec<String>>,
    widget_state: TableState,
    current_sort:ListStatus,
}

impl NewSceneTrait for MainList {

    fn widget<B:Backend>(&mut self, f: &mut Frame<B>){
        let size = f.size();
        let mut table_vector = Vec::new();
        for x in 0 .. self.widget_table.len() {
            table_vector.push(Row::new(self.widget_table[x].clone()));
        }
        let table_widget = Table::new(table_vector)
            .header(
                Row::new(vec!["Name","Progress","Score","Type"])
            )
            .widths(&[Constraint::Percentage(60),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10)
            ])
            .highlight_style(
                Style::default().add_modifier(Modifier::ITALIC)
            );


        f.render_stateful_widget(table_widget, size, &mut self.widget_state);
    }
}

impl MainList {
    pub fn default() -> Self {
        Self {
            anime_list: Vec::new(),

            widget_table: Vec::new(),
            widget_state: TableState::default(),
            current_sort: ListStatus::CURRENT,
        }
    }

    pub fn set_widget_strings(&mut self){
        for x in 0 .. self.anime_list.len() {
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