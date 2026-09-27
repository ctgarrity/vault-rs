use crate::vault::Vault;

mod vault;
fn main() {
    let entry = vault::Entry {
        title: String::from("Test"),
        username: String::from("ctg"),
        password: String::from("123"),
        notes: None,
        website: None,
    };

    let mut vault = Vault::new();
    vault.add(entry);
}
