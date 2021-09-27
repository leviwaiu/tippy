use crate::scene::SceneTrait;

pub struct Settings {
    title_style: String,
    auto_change_status: bool,
}

impl SceneTrait for Settings {
    fn show_view(&self) {
        todo!()
    }

    fn format_status_row(&self) -> String {
        todo!()
    }
}

impl Settings {
    pub fn default() -> Self {
        Self {
            title_style: String::from("native"),
            auto_change_status: true,
        }
    }

    pub fn draw_interface() {

    }

    pub fn auto_change_status(&self) -> bool {
        self.auto_change_status
    }
}