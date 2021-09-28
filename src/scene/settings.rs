use crate::scene::SceneTrait;
use crate::terminal::Terminal;

pub struct Settings {
    title_style: String,
    auto_change_status: bool,
}

// impl SceneTrait for Settings {
//     fn show_view(&self) {
//
//     }
//
//     fn format_status_row(&self) -> String {
//         todo!()
//     }
//
//     fn set_terminal(&mut self, terminal:Terminal) {
//         todo!()
//     }
// }

impl Settings {
    pub fn default() -> Self {
        Self {
            title_style: String::from("native"),
            auto_change_status: true,
        }
    }


    pub fn auto_change_status(&self) -> bool {
        self.auto_change_status
    }
}