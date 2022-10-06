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
        let Table = Table::new();

    }
}

impl MainList {
    pub fn default() -> Self {
        Self {
            anime_list: Vec::new(),
            current_sort: ListStatus::CURRENT,
        }
    }

    pub fn create_string(&self) {

    }

    pub fn get_anime_list(&self) -> Vec<ListEntry> {
        self.anime_list.clone()
    }

    pub fn set_anime_list(&mut self, list: Vec<ListEntry>) {
        self.anime_list = list;
    }
}