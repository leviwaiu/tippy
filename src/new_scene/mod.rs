pub mod mainlist;
pub mod test_struct;


use tui::{
    backend::Backend,
    Frame
};

pub(crate) enum NewScene {
    MainList(),

}

pub trait NewSceneTrait {
    fn widget<B: Backend>(&self, f: &mut Frame<B>);
    
}

impl NewSceneTrait for NewScene {
    fn widget<B:Backend>(&self, f: &mut Frame<B>){

    }

}
