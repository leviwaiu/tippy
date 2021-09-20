#[derive(Clone)]
pub struct Entry{
    id: usize,
    title: String,
    watched_count: usize,
    total_count: usize,
    status: EntryStatus,
    score: u8,
}

#[derive(Clone, Eq, PartialEq)]
pub enum EntryStatus {
    CURRENT,
    PLANNING,
    COMPLETED,
    DROPPED,
    PAUSED,
    REPEATING
}

impl EntryStatus {
    pub fn to_description(&self) -> String {
         match self {
             Self::CURRENT => "Watching",
             Self::PLANNING => "Plan to Watch",
             Self::COMPLETED => "Completed",
             Self::DROPPED => "Dropped",
             Self::PAUSED => "Paused",
             Self::REPEATING => "Watching (R)",
        }.to_string()
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::CURRENT => "CURRENT",
            Self::PLANNING => "PLANNING",
            Self::COMPLETED => "COMPLETED",
            Self::DROPPED => "DROPPED",
            Self::PAUSED => "PAUSED",
            Self::REPEATING => "REPEATING",
        }.to_string()
    }

    pub fn from_string(str:&str) -> Option<Self> {
        match str {
            "CURRENT" => Some(Self::CURRENT),
            "PLANNING" => Some(Self::PLANNING),
            "COMPLETED" => Some(Self::COMPLETED),
            "DROPPED" => Some(Self::DROPPED),
            "PAUSED" => Some(Self::PAUSED),
            "REPEATING" => Some(Self::REPEATING),
            _ => None,
        }
    }
}

impl Entry{
    pub fn default() -> Self {
        Entry{
            id:10,
            title: "This is only a test".to_string(),
            watched_count:10,
            total_count:10,
            status: EntryStatus::PLANNING,
            score:0,
        }
    }

    pub fn new(id:u64, title: String, watched_count:u64, total_count:u64, entry_type:EntryStatus, score:u64 ) -> Self {
        Self{
            id: id as usize,
            title,
            watched_count: watched_count as usize,
            total_count: total_count as usize,
            status: entry_type,
            score: score as u8,
        }
    }



    fn set_watched(&mut self, count: usize){
        if count <= self.total_count{
            self.watched_count = count;
        }
    }

    pub fn add_watched(&mut self){
        if self.watched_count < self.total_count{
            self.watched_count +=1 ;
        }
    }

    pub fn remove_watched(&mut self){
        self.watched_count = self.watched_count.saturating_sub(1);
    }

    pub fn id(&self) -> usize {
        self.id
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
    pub fn status(&self) -> EntryStatus {
        self.status.clone()
    }
    pub fn score(&self) -> u8 {
        self.score
    }

    pub fn set_id(&mut self, id: usize) {
        self.id = id;
    }
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }
    pub fn set_watched_count(&mut self, watched_count: usize) {
        self.watched_count = watched_count;
    }
    pub fn set_total_count(&mut self, total_count: usize) {
        self.total_count = total_count;
    }
    pub fn set_status(&mut self, status: EntryStatus) {
        self.status = status;
    }
    pub fn set_score(&mut self, score: u8) {
        self.score = score;
    }
}