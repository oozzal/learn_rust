use serde::Deserialize;
use std::fmt;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

const API_URL: &str = "https://official-joke-api.appspot.com/jokes/random";

fn request() -> Result<String, reqwest::Error> {
    let out = reqwest::blocking::get(API_URL)?.text()?;
    Ok(out)
}

#[derive(Deserialize, Debug)]
struct Joke {
    #[serde(rename = "type")]
    kind: String,
    setup: String,
    punchline: String,
    id: u16,
}

impl fmt::Display for Joke {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n-> {}", self.setup, self.punchline)
    }
}

pub fn main() {
    let time = Instant::now();
    let (tx, rx) = mpsc::channel();
    let mut results = vec![];
    for _ in 0..2 {
        let cloned_tx = tx.clone();
        thread::spawn(move || {
            cloned_tx.send(request()).unwrap();
        });
    }
    thread::spawn(move || {
        tx.send(request()).unwrap();
    });
    for received in rx {
        let joke_str = received.unwrap();
        let joke: Joke = serde_json::from_str(&joke_str).expect("error");
        results.push(joke);
    }
    for joke in results {
        println!("{}\n", joke);
    }
    println!("DONE IN {:?}", time.elapsed())
}
