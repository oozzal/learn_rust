use std::sync::mpsc;
use std::thread;
use std::time::Instant;

type Int = u16;

fn solve() -> Int {
    let number: Int = rand::random();
    let mut answer = 0;

    for i in 0..=Int::MAX {
        if i == number {
            answer = number
        }
    }
    answer
}

fn recv() -> Vec<Int> {
    let (tx, rx) = mpsc::channel();
    let mut results = vec![];
    for _ in 0..10000 {
        let cloned_tx = tx.clone();
        thread::spawn(move || {
            cloned_tx.send(solve()).unwrap();
        });
        results.push(rx.recv().unwrap());
    }
    results
}

fn received() -> Vec<Int> {
    let (tx, rx) = mpsc::channel();
    let mut results = vec![];
    for _ in 0..10000 {
        let cloned_tx = tx.clone();
        thread::spawn(move || {
            cloned_tx.send(solve()).unwrap();
        });
    }
    thread::spawn(move || {
        tx.send(0).unwrap();
    });
    for received in rx {
        results.push(received);
    }
    results
}

pub fn main() {
    let start = Instant::now();
    recv();
    println!("SOLVED recv() in {:?}!", start.elapsed());

    let start = Instant::now();
    received();
    println!("SOLVED received() in {:?}!", start.elapsed());
}
