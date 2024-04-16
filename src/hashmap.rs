use std::collections::HashMap;

pub fn main() {
    let mut people = HashMap::new();
    people.insert("Uzzal".to_owned(), 32);
    println!("{:?}", people.get("jpt"));
}
