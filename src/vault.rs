pub struct Entry {
    pub title: String,
    pub username: String,
    pub password: String,
    pub website: Option<String>,
    pub notes: Option<String>,
}

pub struct Vault {
    entries: Vec<Entry>,
}

impl Vault {
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add(&mut self, entry: Entry) {
        self.entries.push(entry);
    }
}
