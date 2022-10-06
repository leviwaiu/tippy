use crate::list_entry::{ListEntry, ListStatus};
use crate::new_scene::NewSceneTrait;

use tui::{
    backend::Backend,
    Frame,
};
use tui::widgets::Table;

pub struct MainList {
    anime_list: Vec<ListEntry>,
    current_sort:ListStatus,
}

impl NewSceneTrait for MainList {

    fn widget<B:Backend>(&self, f: &mut Frame<B>){
        let size = f.size();
        let mut table_vector = Vec::new();
        for x in 0 .. self.anime_list.len() {
            table_vector.push(self.create_string(x));
        }
    }
}

impl MainList {
    pub fn default() -> Self {
        Self {
            anime_list: Vec::new(),
            current_sort: ListStatus::CURRENT,
        }
    }

    pub fn create_string(&self, index: usize) -> Vec<String> {
        if index >= self.anime_list.len() {
            return Vec::new();
        }
        let watchcount = format!("{}/{}",
            self.anime_list[index].watched_count(),
            self.anime_list[index].total_count());
        let output = vec!(
            self.anime_list[index].title().to_string(),
            watchcount,
            self.anime_list[index].score().to_string()
        );
        output
    }

    pub fn get_anime_list(&self) -> Vec<ListEntry> {
        self.anime_list.clone()
    }

    pub fn set_anime_list(&mut self, list: Vec<ListEntry>) {
        self.anime_list = list;
    }
}