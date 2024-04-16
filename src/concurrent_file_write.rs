use std::fs::File;
use std::io::{Result, Write};
use std::sync::{Arc, Mutex};
use std::thread::spawn;

fn write(file: &mut File, index: i16) -> Result<()> {
    file.write_all(format!("Writing...{}\n", index).as_bytes())?;
    Ok(())
}

pub fn main() -> Result<()> {
    let file = Arc::new(Mutex::new(File::create("readme.txt")?));
    let mut handlers = vec![];
    for i in 0..i16::MAX {
        let file_clone = Arc::clone(&file);
        let handle = spawn(move || {
            write(&mut file_clone.lock().unwrap(), i).unwrap();
        });
        handlers.push(handle)
    }
    for handle in handlers {
        handle.join().unwrap();
    }
    Ok(())
}
