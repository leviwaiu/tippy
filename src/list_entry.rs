
use tui::widgets::List;
use crate::list_entry::ListStatus::{COMPLETED, CURRENT, DROPPED, PAUSED, PLANNING, REPEATING};

#[derive(Clone)]
pub struct ListEntry {
    id: usize,
    media_id: usize,
    title: String,
    title_length: Option<usize>,
    watched_count: usize,
    total_count: usize,
    status: ListStatus,
    score: u8,
}

#[derive(Clone, Eq, PartialEq)]
pub enum ListStatus {
    CURRENT,
    PLANNING,
    COMPLETED,
    DROPPED,
    PAUSED,
    REPEATING,
}

impl ListStatus {
    pub fn to_description(&self) -> String {
        match self {
            CURRENT => "Watching",
            PLANNING => "Plan to Watch",
            COMPLETED => "Completed",
            DROPPED => "Dropped",
            PAUSED => "Paused",
            REPEATING => "Watching (R)",
        }
        .to_string()
    }

    pub fn to_string(&self) -> String {
        match self {
            CURRENT => "CURRENT",
            PLANNING => "PLANNING",
            COMPLETED => "COMPLETED",
            DROPPED => "DROPPED",
            PAUSED => "PAUSED",
            REPEATING => "REPEATING",
        }
        .to_string()
    }

    pub fn from_string(str: &str) -> Option<Self> {
        match str {
            "CURRENT" => Some(CURRENT),
            "PLANNING" => Some(PLANNING),
            "COMPLETED" => Some(COMPLETED),
            "DROPPED" => Some(DROPPED),
            "PAUSED" => Some(PAUSED),
            "REPEATING" => Some(REPEATING),
            _ => None,
        }
    }

    pub fn from_description(str: &str) -> Option<Self> {
        match str {
            "Watching" => Some(CURRENT),
            "Plan to Watch" => Some(PLANNING),
            "Completed" => Some(COMPLETED),
            "Dropped" => Some(DROPPED),
            "Paused" => Some(PAUSED),
            "Watching (R)" => Some(REPEATING),
            _ => None,
        }
    }

    pub fn create_vec() -> Vec<ListStatus> {
        Vec::from([CURRENT, PLANNING, COMPLETED, DROPPED, PAUSED, REPEATING])
    }
}

impl ListEntry {
    pub fn new(
        id: u64,
        media_id: u64,
        title: String,
        watched_count: u64,
        total_count: u64,
        entry_type: ListStatus,
        score: u64,
    ) -> Self {
        Self {
            id: id as usize,
            media_id: media_id as usize,
            title,
            title_length: None,
            watched_count: watched_count as usize,
            total_count: total_count as usize,
            status: entry_type,
            score: score as u8,
        }
    }

    pub fn add_watched(&mut self) {
        if self.watched_count < self.total_count {
            self.watched_count += 1;
        }
    }

    pub fn remove_watched(&mut self) {
        self.watched_count = self.watched_count.saturating_sub(1);
    }

    pub fn id(&self) -> usize {
        self.id
    }
    pub fn media_id(&self) -> usize {
        self.media_id
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn watched_count(&self) -> usize {
        self.watched_count
    }
    pub fn total_count(&self) -> usize {
        self.total_count
    }
    pub fn status(&self) -> ListStatus {
        self.status.clone()
    }
    pub fn score(&self) -> u8 {
        self.score
    }

    pub fn set_status(&mut self, status: ListStatus) {
        self.status = status;
    }
}
