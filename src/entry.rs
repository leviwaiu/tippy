pub struct Entry{
    pub id: usize,
    pub title: String,
    pub watched_count: usize,
    pub total_count: usize,
    pub entry_type: String,
}

impl Entry{
    pub fn default() -> Self {
        Entry{
            id:10,
            title: "This is only a test".to_string(),
            watched_count:10,
            total_count:10,
            entry_type: "Test".to_string(),
        }
    }

    fn set_watched(mut self, count: usize){
        if count <= self.total_count{
            self.watched_count = count;
        }
    }

    fn add_watched(mut self){
        if self.watched_count < self.total_count{
            self.watched_count +=1 ;
        }
    }
}