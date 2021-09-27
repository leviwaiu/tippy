use std::any::Any;
use crate::scene::mainlist::MainList;

pub mod mainlist;
pub mod settings;
mod anime_info;

pub(crate) enum Scene {
    MainList(MainList),
}

pub trait SceneTrait {
    fn show_view(&self);

    fn format_status_row(&self) -> String;

}

impl SceneTrait for Scene {
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
}