#[derive(Debug)]
struct Entry {
    url: String,
    username: String,
    password: String,
    note: Option<String>,
}

struct Vault {
    entries: Vec<Entry>,
}

impl Vault {
    fn list(&self) {
        for entry in &self.entries[..] {
            println!("{entry:?}");
        }
    }
}

fn main() {
    println!("Hello, world!");
    let e1 = Entry {
        url: String::from("www.google.com"),
        username: String::from("conman"),
        password: String::from("1234"),
        note: None,
    };

    let mut v1 = Vault {
        entries: Vec::new(),
    };

    v1.entries.push(e1);
    v1.list();
}
