pub struct Settings {
    pub title_style: String,
    pub auto_change_status: bool,
}

impl Settings {
    pub fn default() -> Self {
        Self {
            title_style: String::from("native"),
            auto_change_status: true,
        }
    }
}