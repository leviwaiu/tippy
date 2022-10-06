use tui::{
    Frame,
    backend::{Backend, CrosstermBackend},
    widgets::{Widget, Block, Borders},
};

use crate::new_scene::NewSceneTrait;

pub struct TestStruct{
    string: String,

}

impl NewSceneTrait for TestStruct {
    fn widget<B:Backend>(&self, f: &mut Frame<B>){
        let size = f.size();
        let block = Block::default().title(self.string.clone()).borders(Borders::ALL);
        f.render_widget(block, size);
    }
}

impl TestStruct {
    pub fn default()-> Self {
        Self{
            string:"Hello Im A Test string".to_string(),
        }
    }
}