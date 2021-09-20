pub struct Settings {
    title_style: String,
    auto_change_status: bool,
}

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