use crate::scene::mainlist::MainList;
use crate::terminal::Terminal;

pub mod mainlist;
pub mod settings;
mod anime_info;

pub(crate) enum Scene<'term> {
    MainList(MainList<'term>),
}

pub trait SceneTrait<'a>{
    fn show_view(&self);

    fn format_status_row(&self) -> String;

    fn set_terminal(&mut self, terminal: &'a Terminal);
}

impl<'scene> SceneTrait<'scene> for Scene<'scene> {
    fn show_view(&self) {
        match self{
            Scene::MainList(mainlist) => mainlist.show_view()
        }
    }

    fn format_status_row(&self) -> String {
        match self {
            Scene::MainList(main_list) => main_list.format_status_row()
        }
    }

    fn set_terminal(&mut self, terminal: &'scene Terminal) {
        match self {
            Scene::MainList(main_list) => main_list.set_terminal(terminal)
        }
    }
}