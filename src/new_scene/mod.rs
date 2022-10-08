pub mod main_list;
pub mod anime_search;


use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    Frame
};

pub(crate) enum NewScene {
    MainList(),

}

pub trait NewSceneTrait {
    fn widget<B: Backend>(&mut self, f: &mut Frame<B>);

    fn process_key(&mut self, key:KeyCode);
}

impl NewSceneTrait for NewScene {
    fn widget<B:Backend>(&mut self, f: &mut Frame<B>){

    }

    fn process_key(&mut self, key:KeyCode) {
        todo!()
    }

}
