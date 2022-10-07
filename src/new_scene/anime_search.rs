use tui::{
    Frame,
    backend::{Backend, CrosstermBackend},
    widgets::{Widget, Block, Borders},
};

use crate::new_scene::NewSceneTrait;

pub struct AnimeSearch {
    string: String,

}

impl NewSceneTrait for AnimeSearch {
    fn widget<B:Backend>(&mut self, f: &mut Frame<B>){
        let size = f.size();
        let block = Block::default().title(self.string.clone()).borders(Borders::ALL);
        f.render_widget(block, size);
    }
}

impl AnimeSearch {
    pub fn default()-> Self {
        Self{
            string:"Hello Im A Test string".to_string(),
        }
    }
}