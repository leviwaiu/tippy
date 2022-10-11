
use crossterm::event::KeyCode;
use tui::{
    Frame,
    backend::{Backend, CrosstermBackend},
    widgets::{Widget, Block, Borders},
};
use tui::layout::{Constraint, Direction, Layout};

use crate::new_scene::Displayable;

pub struct AnimeSearch {
    keyword: String,


}

impl Displayable for AnimeSearch {
    fn widget<B:Backend>(&mut self, f: &mut Frame<B>){

        let layout = Layout::default().direction(Direction::Vertical)
            .constraints([
                Constraint::Length(6),
                Constraint::Percentage(100),
            ]).split(f.size());

        let size = f.size();

        let block = Block::default().title(self.keyword.clone()).borders(Borders::ALL);
        f.render_widget(block, size);
    }

    fn process_key(&mut self, key: KeyCode) {
        todo!()
    }
}

impl AnimeSearch {
    pub fn default()-> Self {
        Self{
            keyword:String::new(),
        }
    }
}