pub mod main_list;
pub mod anime_search;


use tui::{
    backend::Backend,
    Frame
};

pub(crate) enum NewScene {
    MainList(),

}

pub trait NewSceneTrait {
    fn widget<B: Backend>(&mut self, f: &mut Frame<B>);
    
}

impl NewSceneTrait for NewScene {
    fn widget<B:Backend>(&mut self, f: &mut Frame<B>){

    }

}
