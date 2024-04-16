use std::fmt::Debug;

fn print<T: AsRef<str> + Debug>(input: T) {
    println!("{:?}", input)
}

pub fn main() {
    print("Hello");
    print("Hello".to_string());
}
