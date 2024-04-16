use std::sync::{Arc, Mutex};

pub fn main() {
    let start = Arc::new(Mutex::new(0));
    let mut handle_vec = vec![];
    for _ in 0..10 {
        let start_cloned = Arc::clone(&start);
        let handle = std::thread::spawn(move || {
            *start_cloned.lock().unwrap() += 10;
        });
        handle_vec.push(handle);
    }
    for handle in handle_vec {
        handle.join().unwrap();
    }
    println!("{:?}", start);
}
